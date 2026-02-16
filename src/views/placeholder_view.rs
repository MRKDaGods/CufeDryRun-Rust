use super::View;

pub struct PlaceholderView;

impl View for PlaceholderView {
    fn name(&self) -> &str {
        "Placeholder"
    }

    fn on_show(&self) {
        println!("Placeholder::hello")
    }

    fn on_hide(&self) {
        println!("Placeholder::bye")
    }

    fn on_gui(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Placeholder View");
        });
    }
}
