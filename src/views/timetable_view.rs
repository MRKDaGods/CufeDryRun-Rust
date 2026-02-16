use super::View;

pub struct TimeTableView;

impl View for TimeTableView {
    fn name(&self) -> &str {
        "TimeTable"
    }

    fn on_show(&self) {
        println!("TimeTableView::hello")
    }

    fn on_hide(&self) {
        println!("TimeTableView::bye")
    }

    fn on_gui(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("TimeTable View");
            ui.label("ngl i dont even like arb markets anyway");
        });
    }
}
