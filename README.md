# Spectral Viewer v0.3.1

![Spectral Viewer's UI](https://github.com/lummasharp/Spectral-Viewer/blob/main/docs/media/ui.png)

A small, native desktop image viewer written in Rust. It focuses on opening local images quickly and browsing a folder without editing tools.

## Features

- Open images from the windows context menu or application.
- Browse supported images in the same folder
- Smooth cursor-centered mouse-wheel zoom and click-drag panning
- Switchable smooth and nearest-neighbor image scaling
- Fit-to-window and 100% views
- Rotate and horizontal/vertical flip without modifying the file.
- Fullscreen mode
- Escape key exits fullscreen mode
- EXIF orientation handling
- Clear file name, folder position, zoom level, image metadata, and load errors
- Background image decoding, adjacent-image preloading, and a bounded image cache
- Automatic GitHub release checks and one-click background installation
- Currently supports only Windows

Supported by default: AVIF, BMP, DDS, OpenEXR, GIF, HDR, ICO, JPEG, PNG/PNM, QOI, SVG, TGA, TIFF, and WebP.

## Build and run

Install the current stable Rust toolchain from <https://rustup.rs/>, then:

```sh
cargo run --release
```

Open a specific image:

```sh
cargo run --release -- path/to/image.png
```

Build a standalone optimized executable:

```sh
cargo build --release
```

The executable is written to `target/release/spectral-viewer` (`spectral-viewer.exe` on Windows).

## Build the Windows installer

Install [Inno Setup 6](https://jrsoftware.org/isinfo.php), then run:

```powershell
.\scripts\build-installer.ps1
```

The installer is written to `dist\SpectralViewer-Setup-0.3.0.exe`. It installs per user without requiring administrator access. Spectral Viewer is registered as an available **Open with** and Default Apps candidate for supported image types without changing the user's defaults. During installation, users can optionally add a separate **Open with Spectral Viewer** context-menu command. The final installer page includes a checkbox to launch Spectral Viewer.

The Windows application and installer icon is sourced from `assets\icon.ico`.

Verify the install, context-menu task, and uninstall behavior:

```powershell
.\scripts\test-installer.ps1
```

## Controls

| Action | Control |
| --- | --- |
| Open image | `Ctrl+O` |
| Previous / next | `Left` / `Right` or `Page Up` / `Page Down` |
| First / last | `Home` / `End` |
| Zoom | Mouse wheel |
| Pan | Click and drag |
| Fit to window | `F` or double-click |
| Actual size | `Ctrl+0` |
| Rotate clockwise | `R` |
| Flip horizontally / vertically | `H` / `V` |
| Toggle smooth / nearest-neighbor scaling | `S` |
| Toggle fullscreen | `F11` |
| Exit fullscreen | `Esc` |

## Test

```sh
cargo test
```

Building AVIF support from source on Windows requires NASM 2.x and Perl. Network loading and editing are intentionally outside the default build.
