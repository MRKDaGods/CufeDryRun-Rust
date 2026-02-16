use super::TITLEBAR_HEIGHT;

const TITLEBAR_PADDING_H: f32 = 12.0;

pub fn render_title_bar(ctx: &egui::Context) {
    egui::TopBottomPanel::top("titlebar")
        .frame(egui::Frame {
            inner_margin: egui::Margin::ZERO,
            fill: ctx.style().visuals.window_fill,
            ..Default::default()
        })
        .exact_height(TITLEBAR_HEIGHT)
        .show_separator_line(false)
        .show(ctx, |ui| {
            // Title bar events on windows
            #[cfg(not(target_arch = "wasm32"))]
            super::desktop::handle_title_bar_events(ctx, ui);

            // Main titlebar pass
            ui.with_layout(
                egui::Layout::left_to_right(egui::Align::Center)
                    .with_cross_align(egui::Align::Center),
                |ui| {
                    // Title
                    ui.add_space(TITLEBAR_PADDING_H);
                    ui.label("Cryn - Ammar Magnus");

                    // Window controls on desktop
                    #[cfg(not(target_arch = "wasm32"))]
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        super::desktop::render_window_controls(ctx, ui)
                    });
                },
            );
        });
}
