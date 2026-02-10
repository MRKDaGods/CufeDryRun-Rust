use super::TITLEBAR_HEIGHT;

const RESIZE_MARGIN: f32 = 8.0;

// Control button characters
// segmdl2.ttf
const CLOSE_CHAR: &str = "";
const MAXIMIZE_CHAR: &str = "";
const RESTORE_CHAR: &str = "";
const MINIMIZE_CHAR: &str = "";

pub fn handle_title_bar_events(ctx: &egui::Context, ui: &mut egui::Ui) {
    let title_bar_rect = ui.max_rect();
    let response = ui.interact(
        title_bar_rect,
        egui::Id::new("titlebar_click_drag"),
        egui::Sense::click_and_drag(),
    );

    if response.double_clicked() {
        toggle_maximize(ctx);
    } else if response.drag_started() {
        ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
    }
}

pub fn handle_resize_events(ctx: &egui::Context) {
    // Dont handle resize if window is maximized
    if is_maximized(ctx) {
        return;
    }

    let screen_rect = ctx.available_rect();
    let resize_rects = calculate_resize_rects(screen_rect);

    egui::Area::new(egui::Id::new("resize_area"))
        .fixed_pos(egui::pos2(0.0, 0.0))
        .interactable(true)
        .order(egui::Order::Background)
        .show(ctx, |ui| {
            for (i, rect) in resize_rects.iter().enumerate() {
                handle_single_resize_edge(ctx, ui, *rect, i);
            }
        });
}

pub fn render_window_controls(ctx: &egui::Context, ui: &mut egui::Ui) {
    // Close
    render_control_button(
        ctx,
        ui,
        CLOSE_CHAR,
        egui::Color32::from_rgb(232, 17, 35),
        13.0,
        egui::ViewportCommand::Close,
    );

    // Max/restore
    let is_maximized = is_maximized(ctx);
    render_control_button(
        ctx,
        ui,
        if is_maximized {
            RESTORE_CHAR
        } else {
            MAXIMIZE_CHAR
        },
        egui::Color32::from_rgb(61, 61, 61),
        9.0,
        egui::ViewportCommand::Maximized(!is_maximized),
    );

    // Minimize
    render_control_button(
        ctx,
        ui,
        MINIMIZE_CHAR,
        egui::Color32::from_rgb(61, 61, 61),
        9.0,
        egui::ViewportCommand::Minimized(true),
    );
}

fn render_control_button(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    label: &str,
    color: egui::Color32,
    text_size: f32,
    viewport_cmd: egui::ViewportCommand,
) {
    ui.scope(|ui| {
        let style = ui.style_mut();
        style.visuals.widgets.inactive.bg_fill = egui::Color32::TRANSPARENT;
        style.visuals.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
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
                egui::Button::new(egui::RichText::from(label).size(text_size)).corner_radius(0.0),
            )
            .clicked()
        {
            ctx.send_viewport_cmd(viewport_cmd);
        }
    });
}

fn toggle_maximize(ctx: &egui::Context) {
    let is_maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
    ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!is_maximized));
}

fn is_maximized(ctx: &egui::Context) -> bool {
    ctx.input(|i| i.viewport().maximized.unwrap_or(false))
}

fn calculate_resize_rects(screen_rect: egui::Rect) -> [egui::Rect; 4] {
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

fn handle_single_resize_edge(
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
        ctx.set_cursor_icon(match edge_index {
            0 => egui::CursorIcon::ResizeWest,
            1 => egui::CursorIcon::ResizeEast,
            2 => egui::CursorIcon::ResizeNorth,
            3 => egui::CursorIcon::ResizeSouth,
            _ => egui::CursorIcon::Default,
        });
    }

    if response.drag_started() {
        ctx.send_viewport_cmd(egui::ViewportCommand::BeginResize(match edge_index {
            0 => egui::ResizeDirection::West,
            1 => egui::ResizeDirection::East,
            2 => egui::ResizeDirection::North,
            3 => egui::ResizeDirection::South,
            _ => egui::ResizeDirection::East,
        }));
    }
}
