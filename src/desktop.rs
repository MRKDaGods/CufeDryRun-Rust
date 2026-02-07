use cryn_rs::CrynApp;

pub fn run() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_decorations(false)
            .with_transparent(true),
        ..Default::default()
    };
    eframe::run_native(
        "Cryn",
        options,
        Box::new(|cc| Ok(Box::new(CrynApp::new(cc)))),
    )
    .expect("Failed to start desktop app");
}
