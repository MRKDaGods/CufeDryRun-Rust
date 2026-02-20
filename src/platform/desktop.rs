use cryn_rs::CrynApp;

// 1300x800 on 1920x1080
const DIMS: (f32, f32) = (1300.0, 800.0);

pub fn run() {
    // Scale dims to window
    let window_size = display_info::DisplayInfo::all()
        .ok()
        .and_then(|displays| {
            displays
                .into_iter()
                .find(|display| display.is_primary)
                .map(|display| {
                    (
                        DIMS.0 * (display.width as f32) / 1920.0,
                        DIMS.1 * (display.height as f32) / 1080.0,
                    )
                })
        })
        .unwrap_or(DIMS);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(window_size)
            .with_decorations(false),

        renderer: eframe::Renderer::Glow, // wow
        vsync: false,
        ..Default::default()
    };
    eframe::run_native(
        "Cryn",
        options,
        Box::new(|cc| Ok(Box::new(CrynApp::new(cc)))),
    )
    .expect("Failed to start desktop app");
}
