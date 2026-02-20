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

    fn on_gui(&self, ui: &mut egui::Ui, app_ctx: &CrynContext) {
        let definitions = &app_ctx.course_manager.borrow().course_definitions;

        if definitions.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.heading("No courses found :(");
            });
            return;
        }

        egui::ScrollArea::vertical()
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
            .auto_shrink(false)
            .show(ui, |ui| {
                for i in 0..definitions.len() {
                    ui.label(format!("{}: {}", i, definitions[i].borrow().name));
                }

                // egui::Grid::new("courses_grid")
                //     .num_columns(2)
                //     .spacing([10.0, 10.0])
                //     .min_col_width(ui.available_width() / 2.0)
                //     .striped(true)
                //     .show(ui, |ui| {
                //         for def in definitions {
                //             ui.vertical(|ui| {
                //                 ui.heading(&def.borrow().name);
                //                 ui.label(format!("{} credits", 12));
                //             });

                //             ui.vertical(|ui| {
                //                 ui.label(format!("{}", def.borrow().code));
                //             });

                //             ui.end_row();
                //         }
                //     });
            });
    }
}
