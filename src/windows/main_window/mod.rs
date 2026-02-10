mod title_bar;

#[cfg(not(target_arch = "wasm32"))]
mod desktop;

const TITLEBAR_PADDING_H: f32 = 12.0;
const TITLEBAR_HEIGHT: f32 = 40.0;

pub struct MainWindow {}

impl MainWindow {
    pub fn new() -> Self {
        Self {}
    }

    /// Main render method
    pub fn render(&mut self, ctx: &egui::Context) {
        #[cfg(not(target_arch = "wasm32"))]
        desktop::handle_resize_events(ctx);

        // Title bar and window controls
        title_bar::render_title_bar(ctx);
        self.render_content(ctx);
    }

    /// Render the main content
    fn render_content(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.heading("timetable");
            });
        });
    }
}
