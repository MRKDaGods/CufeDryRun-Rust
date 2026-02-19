use super::View;
use crate::CrynContext;

pub struct CoursesView;

impl View for CoursesView {
    fn name(&self) -> &str {
        "Courses"
    }

    fn on_show(&self, _app_ctx: &CrynContext) {
        println!("CoursesView::hello")
    }

    fn on_hide(&self, _app_ctx: &CrynContext) {
        println!("CoursesView::bye")
    }

    fn on_gui(&self, ui: &mut egui::Ui, _app_ctx: &CrynContext) {
        ui.heading("Courses View");
    }
}
