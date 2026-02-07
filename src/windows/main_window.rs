#[cfg(not(target_arch = "wasm32"))]
use egui::Color32;

#[cfg(not(target_arch = "wasm32"))]
const RESIZE_MARGIN: f32 = 8.0;

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
        self.handle_resize_events(ctx);

        self.render_title_bar(ctx);
        self.render_content(ctx);
    }

    /// Render the main window title bar
    fn render_title_bar(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("titlebar")
            .frame(egui::Frame {
                inner_margin: egui::Margin::ZERO,
                fill: ctx.style().visuals.window_fill,
                ..Default::default()
            })
            .exact_height(TITLEBAR_HEIGHT)
            .show(ctx, |ui| {
                // Title bar events on windows
                #[cfg(not(target_arch = "wasm32"))]
                self.handle_title_bar_events(ctx, ui);

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
                            self.render_window_controls(ctx, ui)
                        });
                    },
                );
            });
    }

    /// Render the main content
    fn render_content(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.heading("timetable");
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Render a single control button
    ///
    /// Desktop only
    fn render_control_button(
        &self,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
        label: &str,
        color: Color32,
        text_size: f32,
        viewport_cmd: egui::ViewportCommand,
    ) {
        ui.scope(|ui| {
            let style = ui.style_mut();
            style.visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
            style.visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
            style.visuals.widgets.hovered.bg_fill = color;
            style.visuals.widgets.hovered.weak_bg_fill = color;
            style.visuals.widgets.active.bg_fill = color;
            style.visuals.widgets.active.weak_bg_fill = color;

            // Remove border
            style.visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;
            style.visuals.widgets.hovered.bg_stroke = egui::Stroke::NONE;
            style.visuals.widgets.active.bg_stroke = egui::Stroke::NONE;

            if ui
                .add_sized(
                    egui::vec2(TITLEBAR_HEIGHT, TITLEBAR_HEIGHT),
                    egui::Button::new(egui::RichText::from(label).size(text_size))
                        .corner_radius(0.0),
                )
                .clicked()
            {
                ctx.send_viewport_cmd(viewport_cmd);
            }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Render window control buttons
    ///
    /// Desktop only
    fn render_window_controls(&self, ctx: &egui::Context, ui: &mut egui::Ui) {
        // Close
        self.render_control_button(
            ctx,
            ui,
            "",
            Color32::from_rgb(232, 17, 35),
            13.0,
            egui::ViewportCommand::Close,
        );

        // Max/restore
        let is_maximized = self.is_maximized(ctx);
        self.render_control_button(
            ctx,
            ui,
            if is_maximized { "" } else { "" },
            Color32::from_rgb(61, 61, 61),
            9.0,
            egui::ViewportCommand::Maximized(!is_maximized),
        );

        // Minimize
        self.render_control_button(
            ctx,
            ui,
            "",
            Color32::from_rgb(61, 61, 61),
            9.0,
            egui::ViewportCommand::Minimized(true),
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Handle title bar drag and double click
    ///
    /// Desktop only
    fn handle_title_bar_events(&self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let title_bar_rect = ui.max_rect();
        let response = ui.interact(
            title_bar_rect,
            egui::Id::new("titlebar_click_drag"),
            egui::Sense::click_and_drag(),
        );

        if response.double_clicked() {
            self.toggle_maximize(ctx);
        } else if response.drag_started() {
            ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Toggle maximize/restore
    ///
    /// Desktop only
    fn toggle_maximize(&self, ctx: &egui::Context) {
        let is_maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
        ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!is_maximized));
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Handle window resize interactions at edges
    ///
    /// Desktop only
    fn handle_resize_events(&self, ctx: &egui::Context) {
        // Dont handle resize if window is maximized
        if self.is_maximized(ctx) {
            return;
        }

        let screen_rect = ctx.available_rect();
        let resize_rects = self.calculate_resize_rects(screen_rect);

        egui::Area::new(egui::Id::new("resize_area"))
            .fixed_pos(egui::pos2(0.0, 0.0))
            .interactable(true)
            .order(egui::Order::Background)
            .show(ctx, |ui| {
                for (i, rect) in resize_rects.iter().enumerate() {
                    self.handle_single_resize_edge(ctx, ui, *rect, i);
                }
            });
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Desktop only
    fn calculate_resize_rects(&self, screen_rect: egui::Rect) -> [egui::Rect; 4] {
        [
            // Left
            egui::Rect::from_min_max(
                screen_rect.min,
                egui::pos2(screen_rect.min.x + RESIZE_MARGIN, screen_rect.max.y),
            ),
            // Right
            egui::Rect::from_min_max(
                egui::pos2(screen_rect.max.x - RESIZE_MARGIN, screen_rect.min.y),
                screen_rect.max,
            ),
            // Top
            egui::Rect::from_min_max(
                screen_rect.min,
                egui::pos2(screen_rect.max.x, screen_rect.min.y + RESIZE_MARGIN),
            ),
            // Bottom
            egui::Rect::from_min_max(
                egui::pos2(screen_rect.min.x, screen_rect.max.y - RESIZE_MARGIN),
                screen_rect.max,
            ),
        ]
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Resize window when dragging from edge
    ///
    /// Desktop only
    fn handle_single_resize_edge(
        &self,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
        rect: egui::Rect,
        edge_index: usize,
    ) {
        let response = ui.interact(
            rect,
            egui::Id::new(format!("resize_{}", edge_index)),
            egui::Sense::drag(),
        );

        // Update cursor icon
        if response.hovered() {
            let cursor = self.get_resize_cursor(edge_index);
            ctx.set_cursor_icon(cursor);
        }

        if response.drag_started() {
            let direction = self.get_resize_direction(edge_index);
            ctx.send_viewport_cmd(egui::ViewportCommand::BeginResize(direction));
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Get cursor icon for resize direction
    ///
    /// Desktop only
    fn get_resize_cursor(&self, edge_index: usize) -> egui::CursorIcon {
        match edge_index {
            0 => egui::CursorIcon::ResizeWest,
            1 => egui::CursorIcon::ResizeEast,
            2 => egui::CursorIcon::ResizeNorth,
            3 => egui::CursorIcon::ResizeSouth,
            _ => egui::CursorIcon::Default,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Get resize direction for edge
    ///
    /// Desktop only
    fn get_resize_direction(&self, edge_index: usize) -> egui::ResizeDirection {
        match edge_index {
            0 => egui::ResizeDirection::West,
            1 => egui::ResizeDirection::East,
            2 => egui::ResizeDirection::North,
            3 => egui::ResizeDirection::South,
            _ => egui::ResizeDirection::East,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Is window maximized?
    ///
    /// Desktop only
    fn is_maximized(&self, ctx: &egui::Context) -> bool {
        ctx.input(|i| i.viewport().maximized.unwrap_or(false))
    }
}
