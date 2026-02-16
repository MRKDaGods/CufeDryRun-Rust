use crate::views::{PlaceholderView, TimeTableView, View};
use std::{any::TypeId, collections::HashMap};

mod nav_bar;
mod title_bar;

#[cfg(not(target_arch = "wasm32"))]
mod desktop;

const TITLEBAR_HEIGHT: f32 = 40.0;

pub struct MainWindow {
    views: HashMap<TypeId, Box<dyn View>>,
    current_view_id: Option<TypeId>,
}

impl MainWindow {
    pub fn new() -> Self {
        let mut window = Self {
            views: HashMap::new(),
            current_view_id: None,
        };

        // Register views
        window.register_view(TimeTableView);
        window.register_view(PlaceholderView);

        // TT view by def
        window.switch_to_view::<TimeTableView>();

        window
    }

    fn register_view<V: View + 'static>(&mut self, view: V) {
        self.views.insert(TypeId::of::<V>(), Box::new(view));
    }

    pub fn switch_to_view<V: View + 'static>(&mut self) {
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
            if !current_view.can_hide() {
                return;
            }

            current_view.on_hide();
        }

        // Update to new view
        self.current_view_id = Some(target_id); /* Copied */
        target_view.unwrap().on_show();
    }

    /// Main render method
    pub fn render(&mut self, ctx: &egui::Context) {
        #[cfg(not(target_arch = "wasm32"))]
        desktop::handle_resize_events(ctx);

        // Title bar and window controls
        title_bar::render_title_bar(ctx);

        // Nav bar
        nav_bar::render_nav_bar(self, ctx);

        // Content
        self.render_content(ctx);
    }

    /// Render the main content
    fn render_content(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.heading("timetable");
            });
        });
    }
}
