use crate::CrynContext;
use egui::epaint::MarginF32;

pub trait View {
    /// View name
    fn name(&self) -> &str;

    /// Should we pad the view?
    fn padding(&self) -> Option<MarginF32> {
        None
    }

    /// Shown callback
    fn on_show(&mut self, app_ctx: &CrynContext);

    /// Hidden callback
    fn on_hide(&mut self, app_ctx: &CrynContext);

    /// Can we hide this view?
    fn can_hide(&self, _app_ctx: &CrynContext) -> bool {
        true
    }

    /// Called every frame when the view is active
    fn on_gui(&mut self, ui: &mut egui::Ui, app_ctx: &CrynContext);
}
