use super::View;
use crate::CrynContext;

pub struct PlaceholderView;

impl View for PlaceholderView {
    fn name(&self) -> &str {
        "Placeholder"
    }

    fn on_show(&mut self, _app_ctx: &CrynContext) {
        println!("Placeholder::hello")
    }

    fn on_hide(&mut self, _app_ctx: &CrynContext) {
        println!("Placeholder::bye")
    }

    fn on_gui(&mut self, ui: &mut egui::Ui, _app_ctx: &CrynContext) {
        ui.heading("Placeholder View");
    }
}
