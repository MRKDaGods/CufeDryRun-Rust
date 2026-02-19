use std::any::TypeId;

use super::MainWindow;
use crate::{
    CrynContext,
    views::{CoursesView, PlaceholderView, TimeTableView, View},
};

const NAVBAR_HEIGHT: f32 = 42.0;

// Navbar characters
// segmdl2.ttf
const ICON_CALENDAR: &str = "\u{E787}";
const ICON_LIBRARY: &str = "\u{E8F1}";
const ICON_SETTINGS: &str = "\u{E713}";
const ICON_SCREENSHOT: &str = "\u{E158}";

pub fn render_nav_bar(main_window: &mut MainWindow, ctx: &egui::Context, app_ctx: &CrynContext) {
    let button_width = (ctx.content_rect().width() / 8.0).clamp(100.0, 150.0);
    egui::TopBottomPanel::bottom("navbar")
        .frame(egui::Frame {
            inner_margin: egui::Margin::ZERO,
            fill: ctx.style().visuals.window_fill,
            ..Default::default()
        })
        .exact_height(NAVBAR_HEIGHT)
        .show_separator_line(true)
        .show(ctx, |ui| {
            ui.scope(|ui| {
                let style = ui.style_mut();

                // No button spacing
                style.spacing.item_spacing.x = 0.0;

                // Remove bg
                style.visuals.widgets.inactive.bg_fill = egui::Color32::TRANSPARENT;
                style.visuals.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;

                // Remove border
                style.visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;
                style.visuals.widgets.hovered.bg_stroke = egui::Stroke::NONE;
                style.visuals.widgets.active.bg_stroke = egui::Stroke::NONE;

                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    // Left side buttons
                    render_button_view::<TimeTableView>(
                        main_window,
                        app_ctx,
                        ctx,
                        ui,
                        ICON_CALENDAR,
                        "Time Table",
                        button_width,
                    );
                    render_button_view::<CoursesView>(
                        main_window,
                        app_ctx,
                        ctx,
                        ui,
                        ICON_LIBRARY,
                        "Courses",
                        button_width,
                    );

                    // Right side buttons
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        render_button_view::<PlaceholderView>(
                            main_window,
                            app_ctx,
                            ctx,
                            ui,
                            ICON_SETTINGS,
                            "Settings",
                            button_width,
                        );

                        render_button(
                            main_window,
                            ctx,
                            ui,
                            ICON_SCREENSHOT,
                            "Screenshot",
                            button_width,
                            Some(|_: &mut MainWindow| {}),
                            Some(false),
                        );
                    });
                });
            });
        });
}

fn render_button_view<V: View + 'static>(
    main_window: &mut MainWindow,
    app_ctx: &CrynContext,
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    icon: &str,
    label: &str,
    button_width: f32,
) {
    let is_active = main_window.current_view_id == Some(TypeId::of::<V>());

    render_button(
        main_window,
        ctx,
        ui,
        icon,
        label,
        button_width,
        Some(|mw: &mut MainWindow| mw.switch_to_view::<V>(app_ctx)),
        Some(is_active),
    );
}

fn render_button(
    main_window: &mut MainWindow,
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    icon: &str,
    label: &str,
    button_width: f32,
    on_click: Option<impl FnOnce(&mut MainWindow)>,
    is_active: Option<bool>,
) {
    let fore_color = match is_active {
        Some(true) => ui.style().visuals.strong_text_color(),
        _ => ui.style().visuals.text_color(),
    };

    let mut job = egui::text::LayoutJob::default();

    // Icon
    job.append(
        icon,
        0.0,
        egui::TextFormat {
            font_id: egui::FontId::proportional(14.5),
            color: fore_color,
            line_height: Some(6.0),
            valign: egui::Align::TOP, // Fixup
            ..Default::default()
        },
    );

    // Label
    job.append(
        label,
        8.0,
        egui::TextFormat {
            font_id: egui::FontId::proportional(13.5),
            color: fore_color,
            ..Default::default()
        },
    );

    let response = ui.add_sized(
        egui::vec2(button_width, ui.available_height()),
        egui::Button::new(job).corner_radius(0.0),
    );

    if response.hovered() {
        ctx.set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    if response.clicked() {
        if let Some(on_click) = on_click {
            on_click(main_window);
        }
    }
}
