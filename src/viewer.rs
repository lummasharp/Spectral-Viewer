use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    path::{Path, PathBuf},
    sync::{
        Arc,
        mpsc::{self, Receiver, RecvTimeoutError, Sender, TryRecvError},
    },
    thread,
    time::Duration,
};

use eframe::egui::{
    self, Align2, Color32, ColorImage, Event, FontId, Key, Mesh, MouseWheelUnit, Pos2, Rect, Sense,
    Shape, Stroke, TextureHandle, TextureOptions, Vec2, ViewportCommand, pos2, vec2,
};
use eframe::emath::GuiRounding;
use eframe::epaint::Vertex;
use image::ImageReader;
use image::{ColorType, ImageFormat};
use serde::{Deserialize, Serialize};

const APP_NAME: &str = "Spectral Viewer";
const MIN_ZOOM: f32 = 0.02;
const MAX_ZOOM: f32 = 64.0;
const FIT_PADDING: f32 = 48.0;
const CHECKER_SIZE: f32 = 12.0;
const STORAGE_KEY: &str = "spectral-viewer-preferences";
const CACHE_MAX_BYTES: usize = 256 * 1024 * 1024;
const CACHE_MAX_IMAGES: usize = 16;
const IMAGE_EXTENSIONS: &[&str] = &[
    "bmp", "gif", "ico", "jpeg", "jpg", "pbm", "pgm", "png", "pnm", "ppm", "qoi", "tif", "tiff",
    "webp",
];

#[derive(Clone, Copy, Default)]
struct ViewTransform {
    quarter_turns: u8,
    flip_horizontal: bool,
    flip_vertical: bool,
}

#[derive(Default, Deserialize, Serialize)]
struct Preferences {
    fullscreen: bool,
}

struct DecodedImage {
    pixels: Arc<ColorImage>,
    size: Vec2,
    bytes: usize,
    metadata: ImageMetadata,
}

#[derive(Clone)]
struct ImageMetadata {
    width: usize,
    height: usize,
    format: String,
    color_mode: String,
    file_size: u64,
}

struct DecodeResult {
    path: PathBuf,
    result: Result<DecodedImage, String>,
}

struct ImageDecoder {
    foreground: Sender<PathBuf>,
    preload: Sender<PathBuf>,
    results: Receiver<DecodeResult>,
}

impl ImageDecoder {
    fn new(ctx: egui::Context) -> Self {
        let (foreground_tx, foreground_rx) = mpsc::channel();
        let (preload_tx, preload_rx) = mpsc::channel();
        let (result_tx, result_rx) = mpsc::channel();
        thread::Builder::new()
            .name("spectral-image-decoder".to_owned())
            .spawn(move || decoder_worker(ctx, foreground_rx, preload_rx, result_tx))
            .expect("failed to start image decoder");
        Self {
            foreground: foreground_tx,
            preload: preload_tx,
            results: result_rx,
        }
    }
}

#[derive(Default)]
struct ImageCache {
    images: HashMap<PathBuf, DecodedImage>,
    recently_used: VecDeque<PathBuf>,
    bytes: usize,
}

impl ImageCache {
    fn get(&mut self, path: &Path) -> Option<Arc<ColorImage>> {
        let image = self.images.get(path)?.pixels.clone();
        self.touch(path);
        Some(image)
    }

    fn insert(&mut self, path: PathBuf, image: DecodedImage) {
        if image.bytes > CACHE_MAX_BYTES {
            return;
        }
        if let Some(previous) = self.images.remove(&path) {
            self.bytes -= previous.bytes;
        }
        self.bytes += image.bytes;
        self.images.insert(path.clone(), image);
        self.touch(&path);

        while self.bytes > CACHE_MAX_BYTES || self.images.len() > CACHE_MAX_IMAGES {
            let Some(oldest) = self.recently_used.pop_front() else {
                break;
            };
            if let Some(removed) = self.images.remove(&oldest) {
                self.bytes -= removed.bytes;
            }
        }
    }

    fn size(&self, path: &Path) -> Option<Vec2> {
        self.images.get(path).map(|image| image.size)
    }

    fn metadata(&self, path: &Path) -> Option<ImageMetadata> {
        self.images.get(path).map(|image| image.metadata.clone())
    }

    fn touch(&mut self, path: &Path) {
        self.recently_used.retain(|item| item != path);
        self.recently_used.push_back(path.to_owned());
    }
}

pub struct ViewerApp {
    texture: Option<TextureHandle>,
    image_size: Vec2,
    current_path: Option<PathBuf>,
    folder_images: Vec<PathBuf>,
    current_index: Option<usize>,
    zoom: f32,
    pan: Vec2,
    fit_requested: bool,
    error: Option<String>,
    transform: ViewTransform,
    preferences: Preferences,
    decoder: ImageDecoder,
    cache: ImageCache,
    pending_preloads: HashSet<PathBuf>,
    loading: bool,
    image_metadata: Option<ImageMetadata>,
}

impl ViewerApp {
    pub fn new(cc: &eframe::CreationContext<'_>, initial_path: Option<PathBuf>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        let preferences: Preferences = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, STORAGE_KEY))
            .unwrap_or_default();
        if preferences.fullscreen {
            cc.egui_ctx
                .send_viewport_cmd(ViewportCommand::Fullscreen(true));
        }
        let mut app = Self {
            texture: None,
            image_size: Vec2::ZERO,
            current_path: None,
            folder_images: Vec::new(),
            current_index: None,
            zoom: 1.0,
            pan: Vec2::ZERO,
            fit_requested: true,
            error: None,
            transform: ViewTransform::default(),
            preferences,
            decoder: ImageDecoder::new(cc.egui_ctx.clone()),
            cache: ImageCache::default(),
            pending_preloads: HashSet::new(),
            loading: false,
            image_metadata: None,
        };

        if let Some(path) = initial_path {
            app.open_path(&cc.egui_ctx, path);
        }
        app
    }

    fn open_dialog(&mut self, ctx: &egui::Context) {
        let mut dialog = rfd::FileDialog::new().add_filter("Images", IMAGE_EXTENSIONS);
        if let Some(parent) = self.current_path.as_deref().and_then(Path::parent) {
            dialog = dialog.set_directory(parent);
        }
        if let Some(path) = dialog.pick_file() {
            self.open_path(ctx, path);
        }
    }

    fn open_path(&mut self, ctx: &egui::Context, path: PathBuf) {
        let path = path.canonicalize().unwrap_or(path);
        self.current_path = Some(path.clone());
        self.folder_images = images_in_same_folder(&path);
        self.current_index = self.folder_images.iter().position(|item| item == &path);
        self.texture = None;
        self.image_size = Vec2::ZERO;
        self.image_metadata = None;
        self.zoom = 1.0;
        self.pan = Vec2::ZERO;
        self.fit_requested = true;
        self.error = None;
        self.transform = ViewTransform::default();
        self.loading = true;
        ctx.send_viewport_cmd(ViewportCommand::Title(self.window_title()));

        if let Some(image) = self.cache.get(&path) {
            self.install_image(ctx, &path, image);
        } else if self.decoder.foreground.send(path).is_err() {
            self.loading = false;
            self.error = Some("The image decoder stopped unexpectedly".to_owned());
        }
        self.preload_adjacent();
    }

    fn install_image(&mut self, ctx: &egui::Context, path: &Path, image: Arc<ColorImage>) {
        self.image_size = self
            .cache
            .size(path)
            .unwrap_or_else(|| vec2(image.width() as f32, image.height() as f32));
        self.image_metadata = self.cache.metadata(path);
        self.texture =
            Some(ctx.load_texture(path.to_string_lossy(), image, TextureOptions::NEAREST));
        self.loading = false;
        self.error = None;
    }

    fn poll_decoder(&mut self, ctx: &egui::Context) {
        while let Ok(decoded) = self.decoder.results.try_recv() {
            self.pending_preloads.remove(&decoded.path);
            match decoded.result {
                Ok(image) => {
                    let pixels = image.pixels.clone();
                    self.cache.insert(decoded.path.clone(), image);
                    if self.current_path.as_deref() == Some(decoded.path.as_path()) {
                        self.install_image(ctx, &decoded.path, pixels);
                    }
                }
                Err(error) if self.current_path.as_deref() == Some(decoded.path.as_path()) => {
                    self.loading = false;
                    self.error = Some(format!(
                        "Could not open {}: {error}",
                        decoded.path.display()
                    ));
                }
                Err(_) => {}
            }
        }
    }

    fn preload_adjacent(&mut self) {
        let Some(current) = self.current_index else {
            return;
        };
        let len = self.folder_images.len();
        if len < 2 {
            return;
        }
        for direction in [-1, 1] {
            let path = self.folder_images[wrapped_index(current, direction, len)].clone();
            if !self.cache.images.contains_key(&path)
                && self.pending_preloads.insert(path.clone())
                && self.decoder.preload.send(path.clone()).is_err()
            {
                self.pending_preloads.remove(&path);
            }
        }
    }

    fn navigate(&mut self, ctx: &egui::Context, direction: isize) {
        if self.folder_images.is_empty() {
            return;
        }
        let next = wrapped_index(
            self.current_index.unwrap_or(0),
            direction,
            self.folder_images.len(),
        );
        self.open_path(ctx, self.folder_images[next].clone());
    }

    fn window_title(&self) -> String {
        self.current_path
            .as_deref()
            .and_then(Path::file_name)
            .map(|name| format!("{} - {APP_NAME}", name.to_string_lossy()))
            .unwrap_or_else(|| APP_NAME.to_owned())
    }

    fn fit_to(&mut self, available: Vec2) {
        if self.image_size.x <= 0.0 || self.image_size.y <= 0.0 {
            return;
        }
        self.zoom = fit_zoom(self.display_image_size(), available, FIT_PADDING);
        self.pan = Vec2::ZERO;
        self.fit_requested = false;
    }

    fn display_image_size(&self) -> Vec2 {
        transformed_size(self.image_size, self.transform.quarter_turns)
    }

    fn rotate_clockwise(&mut self) {
        self.transform.quarter_turns = (self.transform.quarter_turns + 1) % 4;
    }

    fn toggle_fullscreen(&mut self, ctx: &egui::Context) {
        self.preferences.fullscreen = !self.preferences.fullscreen;
        ctx.send_viewport_cmd(ViewportCommand::Fullscreen(self.preferences.fullscreen));
    }

    fn zoom_by(&mut self, factor: f32, focus: Pos2, canvas: Rect) {
        let old_zoom = self.zoom;
        self.zoom = (self.zoom * factor).clamp(MIN_ZOOM, MAX_ZOOM);
        self.pan = (focus - canvas.center())
            - ((focus - canvas.center()) - self.pan) * (self.zoom / old_zoom);
        self.fit_requested = false;
    }

    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        if ctx.input(|input| input.key_pressed(Key::O) && input.modifiers.command) {
            self.open_dialog(ctx);
        }
        if ctx.input(|input| input.key_pressed(Key::ArrowRight) || input.key_pressed(Key::PageDown))
        {
            self.navigate(ctx, 1);
        }
        if ctx.input(|input| input.key_pressed(Key::ArrowLeft) || input.key_pressed(Key::PageUp)) {
            self.navigate(ctx, -1);
        }
        if ctx.input(|input| input.key_pressed(Key::Home)) && !self.folder_images.is_empty() {
            self.open_path(ctx, self.folder_images[0].clone());
        }
        if ctx.input(|input| input.key_pressed(Key::End)) && !self.folder_images.is_empty() {
            self.open_path(
                ctx,
                self.folder_images[self.folder_images.len() - 1].clone(),
            );
        }
        if ctx.input(|input| input.key_pressed(Key::F)) {
            self.fit_requested = true;
        }
        if ctx.input(|input| input.key_pressed(Key::F11)) {
            self.toggle_fullscreen(ctx);
        }
        if ctx.input(|input| input.key_pressed(Key::R)) {
            self.rotate_clockwise();
        }
        if ctx.input(|input| input.key_pressed(Key::H)) {
            self.transform.flip_horizontal = !self.transform.flip_horizontal;
        }
        if ctx.input(|input| input.key_pressed(Key::V)) {
            self.transform.flip_vertical = !self.transform.flip_vertical;
        }
        if ctx.input(|input| input.key_pressed(Key::Num0) && input.modifiers.command) {
            self.zoom = 1.0;
            self.pan = Vec2::ZERO;
            self.fit_requested = false;
        }

        let dropped = ctx.input(|input| {
            input
                .raw
                .dropped_files
                .iter()
                .find_map(|file| file.path.clone())
        });
        if let Some(path) = dropped {
            self.open_path(ctx, path);
        }
    }

    fn image_panel(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, canvas: Rect) {
        let response = ui.allocate_rect(canvas, Sense::click_and_drag());
        let painter = ui.painter().with_clip_rect(canvas);
        painter.rect_filled(canvas, 0.0, Color32::from_rgb(17, 18, 20));

        if self.fit_requested {
            self.fit_to(canvas.size());
        }

        if response.dragged() {
            self.pan += response.drag_delta();
            self.fit_requested = false;
            ctx.request_repaint();
        }
        if response.double_clicked() {
            self.fit_requested = true;
        }

        if response.hovered() {
            let zoom_factor = ctx.input(|input| wheel_zoom_factor(&input.events));
            if zoom_factor != 1.0 {
                let pointer = ctx
                    .input(|input| input.pointer.hover_pos())
                    .unwrap_or(canvas.center());
                self.zoom_by(zoom_factor, pointer, canvas);
                ctx.request_repaint();
            }
        }

        if let Some(texture) = &self.texture {
            let display_size = self.display_image_size() * self.zoom;
            let image_min = canvas.center() + self.pan - display_size * 0.5;
            let image_min = if self.zoom >= 1.0 {
                image_min.round_to_pixels(painter.pixels_per_point())
            } else {
                image_min
            };
            let image_rect = Rect::from_min_size(image_min, display_size);
            paint_checkerboard(&painter, image_rect, canvas);
            paint_transformed_image(&painter, texture, image_rect, self.transform);
        } else if self.loading {
            painter.text(
                canvas.center(),
                Align2::CENTER_CENTER,
                "Loading...",
                FontId::proportional(15.0),
                Color32::from_gray(130),
            );
        } else if self.error.is_some() {
            painter.text(
                canvas.center(),
                Align2::CENTER_CENTER,
                "Couldn't load image\nPress Ctrl+O to open another file",
                FontId::proportional(17.0),
                Color32::from_rgb(205, 125, 125),
            );
        } else {
            painter.text(
                canvas.center(),
                Align2::CENTER_CENTER,
                "Drop an image here\nor press Ctrl+O",
                FontId::proportional(17.0),
                Color32::from_gray(130),
            );
        }
    }

    fn overlay_controls(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, canvas: Rect) {
        let painter = ui.painter();
        let title = self
            .current_path
            .as_deref()
            .and_then(Path::file_name)
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| APP_NAME.to_owned());
        painter.text(
            canvas.left_top() + vec2(18.0, 16.0),
            Align2::LEFT_TOP,
            title,
            FontId::proportional(15.0),
            Color32::from_gray(220),
        );

        let status = self.current_index.map_or_else(
            || format!("{:.0}%", self.zoom * 100.0),
            |index| {
                format!(
                    "{} / {}    {:.0}%",
                    index + 1,
                    self.folder_images.len(),
                    self.zoom * 100.0
                )
            },
        );
        painter.text(
            canvas.right_top() + vec2(-18.0, 16.0),
            Align2::RIGHT_TOP,
            status,
            FontId::monospace(13.0),
            Color32::from_gray(165),
        );

        if let Some(metadata) = &self.image_metadata {
            let details = format!(
                "{} x {}\n{}\n{}\n{}",
                metadata.width,
                metadata.height,
                metadata.format,
                metadata.color_mode,
                format_file_size(metadata.file_size)
            );
            let bottom_margin = if canvas.width() < 720.0 { 72.0 } else { 18.0 };
            painter.text(
                canvas.left_bottom() + vec2(18.0, -bottom_margin),
                Align2::LEFT_BOTTOM,
                details,
                FontId::monospace(12.0),
                Color32::from_gray(150),
            );
        }

        let bar_size = vec2(438.0, 42.0);
        let bar = Rect::from_center_size(canvas.center_bottom() - vec2(0.0, 28.0), bar_size);
        painter.rect_filled(bar, 10.0, Color32::from_black_alpha(175));

        let labels = ["Open", "<", "Fit", "1:1", "Rotate", "Flip H", "Flip V", ">"];
        let widths = [54.0, 32.0, 42.0, 42.0, 56.0, 54.0, 54.0, 32.0];
        let mut x = bar.left() + 10.0;
        for (index, (label, width)) in labels.into_iter().zip(widths).enumerate() {
            let rect = Rect::from_min_size(pos2(x, bar.top() + 6.0), vec2(width, 30.0));
            let enabled = !matches!(index, 1 | 7) || !self.folder_images.is_empty();
            let response = ui.put(
                rect,
                egui::Button::new(label)
                    .fill(Color32::TRANSPARENT)
                    .stroke(Stroke::NONE)
                    .sense(if enabled {
                        Sense::click()
                    } else {
                        Sense::hover()
                    }),
            );
            if enabled && response.clicked() {
                match index {
                    0 => self.open_dialog(ctx),
                    1 => self.navigate(ctx, -1),
                    2 => self.fit_requested = true,
                    3 => {
                        self.zoom = 1.0;
                        self.pan = Vec2::ZERO;
                        self.fit_requested = false;
                    }
                    4 => self.rotate_clockwise(),
                    5 => self.transform.flip_horizontal = !self.transform.flip_horizontal,
                    6 => self.transform.flip_vertical = !self.transform.flip_vertical,
                    7 => self.navigate(ctx, 1),
                    _ => {}
                }
            }
            x += width + 2.0;
        }
    }
}

impl eframe::App for ViewerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.poll_decoder(&ctx);
        self.handle_shortcuts(&ctx);
        let canvas = ui.max_rect();
        self.image_panel(&ctx, ui, canvas);
        self.overlay_controls(&ctx, ui, canvas);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, STORAGE_KEY, &self.preferences);
    }
}

fn decoder_worker(
    ctx: egui::Context,
    foreground: Receiver<PathBuf>,
    preload: Receiver<PathBuf>,
    results: Sender<DecodeResult>,
) {
    loop {
        let (mut path, is_foreground) = match foreground.try_recv() {
            Ok(path) => (path, true),
            Err(TryRecvError::Disconnected) => match preload.recv() {
                Ok(path) => (path, false),
                Err(_) => break,
            },
            Err(TryRecvError::Empty) => match preload.recv_timeout(Duration::from_millis(20)) {
                Ok(path) => (path, false),
                Err(RecvTimeoutError::Timeout) => continue,
                Err(RecvTimeoutError::Disconnected) => match foreground.recv() {
                    Ok(path) => (path, true),
                    Err(_) => break,
                },
            },
        };
        if is_foreground {
            while let Ok(newer_path) = foreground.try_recv() {
                path = newer_path;
            }
        }
        let result = load_color_image(&path);
        if results.send(DecodeResult { path, result }).is_err() {
            break;
        }
        ctx.request_repaint();
    }
}

fn load_color_image(path: &Path) -> Result<DecodedImage, String> {
    if !path.is_file() {
        return Err("the path is not a file".to_owned());
    }

    let file_size = fs::metadata(path).map_err(|error| error.to_string())?.len();
    let reader = ImageReader::open(path)
        .map_err(|error| error.to_string())?
        .with_guessed_format()
        .map_err(|error| error.to_string())?;
    let format = reader.format();
    let decoded = reader.decode().map_err(|error| error.to_string())?;
    let color_mode = color_mode_name(decoded.color()).to_owned();
    let rgba = decoded.to_rgba8();
    let (width, height) = rgba.dimensions();
    let width = usize::try_from(width).map_err(|_| "image width is too large")?;
    let height = usize::try_from(height).map_err(|_| "image height is too large")?;
    let bytes = width
        .checked_mul(height)
        .and_then(|pixels| pixels.checked_mul(4))
        .ok_or("image dimensions are too large")?;
    let color_image = ColorImage::from_rgba_unmultiplied([width, height], rgba.as_raw());
    Ok(DecodedImage {
        pixels: Arc::new(color_image),
        size: vec2(width as f32, height as f32),
        bytes,
        metadata: ImageMetadata {
            width,
            height,
            format: format_name(format),
            color_mode,
            file_size,
        },
    })
}

fn format_name(format: Option<ImageFormat>) -> String {
    format
        .map(|format| format!("{format:?}").to_uppercase())
        .unwrap_or_else(|| "UNKNOWN".to_owned())
}

fn color_mode_name(color: ColorType) -> &'static str {
    match color {
        ColorType::L8 => "Grayscale 8-bit",
        ColorType::La8 => "Grayscale + Alpha 8-bit",
        ColorType::Rgb8 => "RGB 8-bit",
        ColorType::Rgba8 => "RGBA 8-bit",
        ColorType::L16 => "Grayscale 16-bit",
        ColorType::La16 => "Grayscale + Alpha 16-bit",
        ColorType::Rgb16 => "RGB 16-bit",
        ColorType::Rgba16 => "RGBA 16-bit",
        ColorType::Rgb32F => "RGB 32-bit float",
        ColorType::Rgba32F => "RGBA 32-bit float",
        _ => "Unknown color mode",
    }
}

fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{bytes} B")
    } else {
        format!("{value:.1} {}", UNITS[unit])
    }
}

fn images_in_same_folder(path: &Path) -> Vec<PathBuf> {
    let Some(parent) = path.parent() else {
        return vec![path.to_owned()];
    };
    let Ok(entries) = fs::read_dir(parent) else {
        return vec![path.to_owned()];
    };

    let mut images: Vec<_> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|candidate| candidate.is_file() && is_supported_extension(candidate))
        .collect();
    images.sort_by(|left, right| {
        left.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_lowercase()
            .cmp(
                &right
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_lowercase(),
            )
    });
    images
}

fn is_supported_extension(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            IMAGE_EXTENSIONS
                .iter()
                .any(|supported| extension.eq_ignore_ascii_case(supported))
        })
}

fn wrapped_index(current: usize, direction: isize, len: usize) -> usize {
    (current as isize + direction).rem_euclid(len as isize) as usize
}

fn fit_zoom(image_size: Vec2, canvas_size: Vec2, padding: f32) -> f32 {
    let usable = (canvas_size - Vec2::splat(padding * 2.0)).max(Vec2::splat(1.0));
    (usable.x / image_size.x)
        .min(usable.y / image_size.y)
        .min(1.0)
        .clamp(MIN_ZOOM, MAX_ZOOM)
}

fn wheel_zoom_factor(events: &[Event]) -> f32 {
    let exponent: f32 = events
        .iter()
        .filter_map(|event| match event {
            Event::MouseWheel { unit, delta, .. } => Some(match unit {
                MouseWheelUnit::Point => delta.y * 0.002,
                MouseWheelUnit::Line => delta.y * 0.12,
                MouseWheelUnit::Page => delta.y * 0.5,
            }),
            _ => None,
        })
        .sum();
    exponent.exp()
}

fn paint_checkerboard(painter: &egui::Painter, image_rect: Rect, canvas: Rect) {
    let visible = image_rect.intersect(canvas);
    if visible.is_negative() {
        return;
    }

    painter.rect_filled(visible, 0.0, Color32::from_rgb(48, 50, 54));
    let start_x = checker_index(visible.left(), canvas.left());
    let end_x = checker_index(visible.right(), canvas.left()) + 1;
    let start_y = checker_index(visible.top(), canvas.top());
    let end_y = checker_index(visible.bottom(), canvas.top()) + 1;

    for y in start_y..end_y {
        for x in start_x..end_x {
            if (x + y).rem_euclid(2) == 0 {
                let tile = Rect::from_min_size(
                    canvas.left_top() + vec2(x as f32 * CHECKER_SIZE, y as f32 * CHECKER_SIZE),
                    Vec2::splat(CHECKER_SIZE),
                )
                .intersect(visible);
                painter.rect_filled(tile, 0.0, Color32::from_rgb(66, 69, 74));
            }
        }
    }
}

fn checker_index(position: f32, canvas_origin: f32) -> i32 {
    ((position - canvas_origin) / CHECKER_SIZE).floor() as i32
}

fn transformed_size(size: Vec2, quarter_turns: u8) -> Vec2 {
    if quarter_turns.is_multiple_of(2) {
        size
    } else {
        vec2(size.y, size.x)
    }
}

fn transformed_uv(position: Pos2, transform: ViewTransform) -> Pos2 {
    let mut uv = position;
    if transform.flip_horizontal {
        uv.x = 1.0 - uv.x;
    }
    if transform.flip_vertical {
        uv.y = 1.0 - uv.y;
    }
    for _ in 0..transform.quarter_turns {
        uv = pos2(uv.y, 1.0 - uv.x);
    }
    uv
}

fn paint_transformed_image(
    painter: &egui::Painter,
    texture: &TextureHandle,
    rect: Rect,
    transform: ViewTransform,
) {
    let normalized = [
        pos2(0.0, 0.0),
        pos2(1.0, 0.0),
        pos2(1.0, 1.0),
        pos2(0.0, 1.0),
    ];
    let positions = [
        rect.left_top(),
        rect.right_top(),
        rect.right_bottom(),
        rect.left_bottom(),
    ];
    let mut mesh = Mesh::with_texture(texture.id());
    mesh.vertices = positions
        .into_iter()
        .zip(normalized)
        .map(|(pos, normalized)| Vertex {
            pos,
            uv: transformed_uv(normalized, transform),
            color: Color32::WHITE,
        })
        .collect();
    mesh.indices = vec![0, 1, 2, 0, 2, 3];
    painter.add(Shape::mesh(mesh));
}

#[cfg(test)]
mod tests {
    use super::{
        DecodedImage, ImageCache, ImageDecoder, Preferences, ViewTransform, ViewerApp,
        checker_index, fit_zoom, format_file_size, images_in_same_folder, is_supported_extension,
        transformed_size, transformed_uv, wheel_zoom_factor, wrapped_index,
    };
    use eframe::egui::{
        Color32, ColorImage, Event, Modifiers, MouseWheelUnit, TouchPhase, pos2, vec2,
    };
    use std::{collections::HashSet, fs, path::Path, sync::Arc};

    #[test]
    fn supported_extensions_are_case_insensitive() {
        assert!(is_supported_extension(Path::new("photo.JPEG")));
        assert!(is_supported_extension(Path::new("photo.webp")));
        assert!(!is_supported_extension(Path::new("notes.txt")));
    }

    #[test]
    fn navigation_wraps_in_both_directions() {
        assert_eq!(wrapped_index(2, 1, 3), 0);
        assert_eq!(wrapped_index(0, -1, 3), 2);
        assert_eq!(wrapped_index(1, 1, 3), 2);
    }

    #[test]
    fn fit_zoom_keeps_large_images_inside_the_padded_canvas() {
        assert_eq!(
            fit_zoom(vec2(2000.0, 1000.0), vec2(1000.0, 700.0), 50.0),
            0.45
        );
        assert_eq!(fit_zoom(vec2(200.0, 100.0), vec2(1000.0, 700.0), 50.0), 1.0);
    }

    #[test]
    fn raw_wheel_zoom_is_symmetric_and_not_repeated_across_frames() {
        let wheel = |delta| Event::MouseWheel {
            unit: MouseWheelUnit::Point,
            delta: vec2(0.0, delta),
            modifiers: Modifiers::NONE,
            phase: TouchPhase::Move,
        };
        let zoom_in = wheel_zoom_factor(&[wheel(120.0)]);
        let zoom_out = wheel_zoom_factor(&[wheel(-120.0)]);

        assert!((zoom_in * zoom_out - 1.0).abs() < 0.000_001);
        assert_eq!(wheel_zoom_factor(&[]), 1.0);
        assert!(zoom_in < 1.3);
    }

    #[test]
    fn checker_alignment_is_anchored_to_the_canvas() {
        assert_eq!(checker_index(25.0, 1.0), 2);
        assert_eq!(checker_index(25.0, 7.0), 1);
    }

    #[test]
    fn rotation_swaps_display_dimensions_on_odd_turns() {
        assert_eq!(transformed_size(vec2(640.0, 480.0), 0), vec2(640.0, 480.0));
        assert_eq!(transformed_size(vec2(640.0, 480.0), 1), vec2(480.0, 640.0));
        assert_eq!(transformed_size(vec2(640.0, 480.0), 2), vec2(640.0, 480.0));
    }

    #[test]
    fn view_transform_maps_texture_coordinates_without_modifying_pixels() {
        let rotated = ViewTransform {
            quarter_turns: 1,
            ..Default::default()
        };
        assert_eq!(transformed_uv(pos2(0.0, 0.0), rotated), pos2(0.0, 1.0));

        let flipped = ViewTransform {
            flip_horizontal: true,
            ..Default::default()
        };
        assert_eq!(transformed_uv(pos2(0.0, 0.0), flipped), pos2(1.0, 0.0));
    }

    #[test]
    fn rotation_preserves_zoom_pan_and_fit_state() {
        let mut app = ViewerApp {
            texture: None,
            image_size: vec2(640.0, 480.0),
            current_path: None,
            folder_images: Vec::new(),
            current_index: None,
            zoom: 3.5,
            pan: vec2(120.0, -45.0),
            fit_requested: false,
            error: None,
            transform: ViewTransform::default(),
            preferences: Preferences::default(),
            decoder: ImageDecoder::new(eframe::egui::Context::default()),
            cache: ImageCache::default(),
            pending_preloads: HashSet::new(),
            loading: false,
            image_metadata: None,
        };

        app.rotate_clockwise();

        assert_eq!(app.transform.quarter_turns, 1);
        assert_eq!(app.zoom, 3.5);
        assert_eq!(app.pan, vec2(120.0, -45.0));
        assert!(!app.fit_requested);
    }

    #[test]
    fn folder_images_are_filtered_and_sorted_case_insensitively() {
        let directory =
            std::env::temp_dir().join(format!("spectral-viewer-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&directory);
        fs::create_dir(&directory).unwrap();
        for name in ["z.png", "A.jpg", "middle.txt"] {
            fs::write(directory.join(name), []).unwrap();
        }

        let images = images_in_same_folder(&directory.join("A.jpg"));
        let names: Vec<_> = images
            .iter()
            .map(|path| path.file_name().unwrap().to_string_lossy().into_owned())
            .collect();

        assert_eq!(names, ["A.jpg", "z.png"]);
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn image_cache_evicts_the_least_recently_used_entry() {
        let mut cache = ImageCache::default();
        let decoded = || DecodedImage {
            pixels: Arc::new(ColorImage::new([1, 1], vec![Color32::WHITE])),
            size: vec2(1.0, 1.0),
            bytes: 4,
            metadata: super::ImageMetadata {
                width: 1,
                height: 1,
                format: "PNG".to_owned(),
                color_mode: "RGBA 8-bit".to_owned(),
                file_size: 4,
            },
        };

        for index in 0..16 {
            cache.insert(format!("{index}.png").into(), decoded());
        }
        assert!(cache.get(Path::new("0.png")).is_some());
        cache.insert("16.png".into(), decoded());

        assert!(cache.get(Path::new("0.png")).is_some());
        assert!(cache.get(Path::new("1.png")).is_none());
        assert!(cache.get(Path::new("16.png")).is_some());
    }

    #[test]
    fn file_sizes_use_compact_binary_units() {
        assert_eq!(format_file_size(999), "999 B");
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(2_621_440), "2.5 MB");
    }
}
