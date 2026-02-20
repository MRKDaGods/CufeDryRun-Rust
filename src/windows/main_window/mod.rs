use crate::{
    CrynContext,
    views::{CoursesView, PlaceholderView, TimeTableView, View},
};
use std::{any::TypeId, collections::HashMap};

mod nav_bar;
mod title_bar;

#[cfg(not(target_arch = "wasm32"))]
mod desktop;

const TITLEBAR_HEIGHT: f32 = 40.0;
const NAVBAR_HEIGHT: f32 = 42.0;
const CONTENT_PADDING: i8 = 16;

pub struct MainWindow {
    views: HashMap<TypeId, Box<dyn View>>,
    current_view_id: Option<TypeId>,
}

impl MainWindow {
    pub fn new(app_ctx: &CrynContext) -> Self {
        let mut window = Self {
            views: HashMap::new(),
            current_view_id: None,
        };

        // Register views
        window.register_view(TimeTableView);
        window.register_view(CoursesView);
        window.register_view(PlaceholderView);

        // TT view by def
        window.switch_to_view::<TimeTableView>(app_ctx);

        window
    }

    fn register_view<V: View + 'static>(&mut self, view: V) {
        self.views.insert(TypeId::of::<V>(), Box::new(view));
    }

    pub fn switch_to_view<V: View + 'static>(&mut self, app_ctx: &CrynContext) {
        let target_id = TypeId::of::<V>();

        // Does target view exist?
        let target_view = self.views.get(&target_id);
        if target_view.is_none() {
            return;
        }

        if self.current_view_id == Some(target_id) {
            return;
        }

        // Hide current
        if let Some(current_view_id) = self.current_view_id {
            let current_view = &self.views[&current_view_id];
            if !current_view.can_hide(app_ctx) {
                return;
            }

            current_view.on_hide(app_ctx);
        }

        // Update to new view
        self.current_view_id = Some(target_id); /* Copied */
        target_view.unwrap().on_show(app_ctx);
    }

    /// Main render method
    pub fn render(&mut self, ctx: &egui::Context, app_ctx: &CrynContext) {
        #[cfg(not(target_arch = "wasm32"))]
        desktop::handle_resize_events(ctx);

        // Title bar and window controls
        title_bar::render_title_bar(ctx, self.get_current_view());

        // Nav bar
        nav_bar::render_nav_bar(self, ctx, app_ctx);

        // Content
        self.render_content(ctx, app_ctx);
    }

    /// Render the main content
    fn render_content(&self, ctx: &egui::Context, app_ctx: &CrynContext) {
        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .inner_margin(egui::Margin::ZERO) // egui::Margin::same(CONTENT_PADDING)
                    .fill(ctx.style().visuals.window_fill),
            )
            .show(ctx, |ui| {
                // Render current view
                if let Some(current_view) = self.get_current_view() {
                    //ui.add_sized(ui.available_size(), egui::Label::new("Loading..."));
                    ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                        current_view.as_ref().on_gui(ui, app_ctx)
                    });
                } else {
                    ui.centered_and_justified(|ui| {
                        ui.heading(match self.current_view_id {
                            Some(current_view_id) => {
                                format!("View {:?} not found", current_view_id)
                            }

                            // No view?
                            None => "No view set".to_owned(),
                        });
                    });
                }
            });
    }

    fn get_current_view(&self) -> Option<&Box<dyn View>> {
        let current_view_id = self
            .current_view_id
            .unwrap_or(TypeId::of::<PlaceholderView>());

        self.views.get(&current_view_id)
    }
}
