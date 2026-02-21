use super::View;
use crate::{
    CrynContext,
    models::{CourseSpan, OrderedWeekday},
    views::CoursesView,
    windows::{MainWindow, Window},
};
use egui::{Align, Label, Layout, RichText, Sense};
use std::collections::BTreeMap;

const TIMESLOT_WIDTH: f32 = 95.0;
const TIMESLOT_HEIGHT: f32 = 43.0;

const DAY_WIDTH: f32 = 158.0;
const DAY_HEIGHT: f32 = 42.0;

pub struct TimeTableView {
    span_map: BTreeMap<OrderedWeekday, CourseSpan>,
}

impl TimeTableView {
    pub fn new() -> Self {
        Self {
            span_map: BTreeMap::new(),
        }
    }
}

impl View for TimeTableView {
    fn name(&self) -> &str {
        "Time Table"
    }

    fn on_show(&mut self, app_ctx: &CrynContext) {
        // Build span map
        let available_records = &app_ctx
            .course_manager
            .borrow()
            .get_available_course_records();

        self.span_map.clear();
        available_records.iter().for_each(|record| {
            self.span_map
                .entry(record.borrow().day.into())
                .or_insert(CourseSpan::new())
                .insert_course_record(record);
        });

        self.span_map.iter().for_each(|(day, span)| {
            println!("{}: {} periods", day.to_string(), span.get_period_count());
        });
    }

    fn on_hide(&mut self, _app_ctx: &CrynContext) {}

    fn on_gui(&mut self, ui: &mut egui::Ui, app_ctx: &CrynContext, window: &mut dyn Window) {
        // hmmm
        // elnas 3yza eh
        // elnas bt3ml eh
        // 3en safra wltanya 7amra
        // ololy a3ml eh
        // 3en safra wltanya khadra ;)
        // ololy a3ml ehhhhhh

        if self.span_map.is_empty() {
            ui.centered_and_justified(|ui| {
                if ui
                    .add(
                        Label::new(RichText::new("Select courses to start").heading())
                            .sense(Sense::click()),
                    )
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                {
                    // Go to courses view
                    if let Some(main_window) = window.as_any_mut().downcast_mut::<MainWindow>() {
                        main_window.switch_to_view::<CoursesView>(app_ctx);
                    }
                }
            });
            return;
        }

        ui.with_layout(
            Layout::left_to_right(Align::Min).with_main_wrap(true),
            |ui| {
                for (day, span) in &self.span_map {
                    let period_count = span.get_period_count();
                    let width = period_count as f32 * TIMESLOT_WIDTH;

                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.add_sized([width, DAY_HEIGHT], Label::new(day.to_string()));
                        });
                    });
                }
            },
        );
    }
}
