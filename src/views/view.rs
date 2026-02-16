pub trait View {
    /// View name
    fn name(&self) -> &str;

    /// Shown callback
    fn on_show(&self);

    /// Hidden callback
    fn on_hide(&self);

    /// Can we hide this view?
    fn can_hide(&self) -> bool {
        true
    }

    /// Called every frame when the view is active
    fn on_gui(&self, ctx: &egui::Context);
}
