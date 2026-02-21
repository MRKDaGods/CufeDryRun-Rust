use crate::{
    CrynContext,
    views::{CoursesView, PlaceholderView, TimeTableView, View},
};
use egui::{Align, CentralPanel, Frame, Layout, epaint::MarginF32};
use std::{any::TypeId, collections::HashMap};

mod nav_bar;
mod title_bar;

#[cfg(not(target_arch = "wasm32"))]
mod desktop;

const TITLEBAR_HEIGHT: f32 = 40.0;
const NAVBAR_HEIGHT: f32 = 42.0;

#[allow(unused)]
pub const CONTENT_PADDING: f32 = 8.0;

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
        window.register_view(CoursesView::new());
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

        if self.current_view_id == Some(target_id) {
            return;
        }

        // Does target view exist?
        let view_exists = self.views.contains_key(&target_id);
        if !view_exists {
            return;
        }

        // Hide current
        if let Some(current_view_id) = self.current_view_id {

            let current_view = &mut self.views.get_mut(&current_view_id).unwrap();
            if !current_view.can_hide(app_ctx) {
                return;
            }

            current_view.on_hide(app_ctx);
        }

        // Update to new view
        self.current_view_id = Some(target_id); /* Copied */
        let target_view = &mut self.views.get_mut(&target_id).unwrap();
        target_view.on_show(app_ctx);
    }

    /// Main render method
    pub fn render(&mut self, ctx: &egui::Context, app_ctx: &CrynContext) {
        #[cfg(not(target_arch = "wasm32"))]
        desktop::handle_resize_events(ctx);

        // Title bar and window controls
        title_bar::render_title_bar(ctx, self.get_current_view().as_deref());

        // Nav bar
        nav_bar::render_nav_bar(self, ctx, app_ctx);

        // Content
        self.render_content(ctx, app_ctx);
    }

    /// Render the main content
    fn render_content(&mut self, ctx: &egui::Context, app_ctx: &CrynContext) {
        let view_padding = self
            .get_current_view()
            .filter(|v| v.padding().is_some())
            .map_or(MarginF32::ZERO, |v| v.padding().unwrap());

        CentralPanel::default()
            .frame(
                Frame::new()
                    .inner_margin(view_padding)
                    .fill(ctx.style().visuals.window_fill),
            )
            .show(ctx, |ui| {
                // Render current view
                if let Some(current_view) = self.get_current_view() {
                    ui.with_layout(Layout::top_down(Align::Center), |ui| {
                        current_view.as_mut().on_gui(ui, app_ctx)
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

    fn get_current_view(&mut self) -> Option<&mut Box<dyn View>> {
        let current_view_id = self
            .current_view_id
            .unwrap_or(TypeId::of::<PlaceholderView>());

        self.views.get_mut(&current_view_id)
    }
}
