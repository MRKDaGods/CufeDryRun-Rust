use super::View;
use crate::CrynContext;

pub struct TimeTableView;

impl View for TimeTableView {
    fn name(&self) -> &str {
        "TimeTable"
    }

    fn on_show(&mut self, _ctx: &CrynContext) {
        println!("TimeTableView::hello")
    }

    fn on_hide(&mut self, _ctx: &CrynContext) {
        println!("TimeTableView::bye")
    }

    fn on_gui(&mut self, ui: &mut egui::Ui, _ctx: &CrynContext) {
        ui.heading("TimeTable View");
        ui.label("ngl i dont even like arb markets anyway");
    }
}
