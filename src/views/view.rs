use crate::CrynContext;

pub trait View {
    /// View name
    fn name(&self) -> &str;

    /// Shown callback
    fn on_show(&self, app_ctx: &CrynContext);

    /// Hidden callback
    fn on_hide(&self, app_ctx: &CrynContext);

    /// Can we hide this view?
    fn can_hide(&self, _app_ctx: &CrynContext) -> bool {
        true
    }

    /// Called every frame when the view is active
    fn on_gui(&self, ui: &mut egui::Ui, app_ctx: &CrynContext);
}
