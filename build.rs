fn main() {
    println!("cargo:rerun-if-changed=assets/icon.ico");

    if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        let mut resource = winresource::WindowsResource::new();
        resource
            .set_icon("assets/icon.ico")
            .set("ProductName", "Spectral Viewer")
            .set("FileDescription", "Spectral Viewer")
            .set("InternalName", "spectral-viewer")
            .set("OriginalFilename", "spectral-viewer.exe");
        resource
            .compile()
            .expect("failed to compile Windows resources");
    }
}
