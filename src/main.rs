#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod viewer;

use std::path::PathBuf;

use eframe::egui;
use viewer::ViewerApp;

fn main() -> eframe::Result {
    let initial_path = std::env::args_os().nth(1).map(PathBuf::from);
    let mut viewport = egui::ViewportBuilder::default()
        .with_app_id("spectral-viewer")
        .with_inner_size([1000.0, 700.0])
        .with_min_inner_size([480.0, 320.0])
        .with_visible(false)
        .with_drag_and_drop(true);
    if let Some(icon) = app_icon() {
        viewport = viewport.with_icon(icon);
    }
    let options = eframe::NativeOptions {
        viewport,
        persist_window: true,
        ..Default::default()
    };

    eframe::run_native(
        "Spectral Viewer",
        options,
        Box::new(move |cc| Ok(Box::new(ViewerApp::new(cc, initial_path)))),
    )
}

fn app_icon() -> Option<egui::IconData> {
    let image = image::load_from_memory(include_bytes!("../assets/icon.ico"))
        .ok()?
        .to_rgba8();
    let (width, height) = image.dimensions();
    Some(egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    })
}
