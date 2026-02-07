use crate::utils;
use crate::windows::MainWindow;
use std::sync::Arc;

pub struct CrynApp {
    main_window: MainWindow,
}

impl CrynApp {
    /// App ctor
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        utils::log("Cryn started");

        // Configure theme
        let mut visuals = egui::Visuals::dark();
        visuals.override_text_color = Some(egui::Color32::WHITE);
        cc.egui_ctx.set_visuals(visuals);

        // Fonts
        Self::setup_fonts_static(cc);

        // Turn off text selection
        cc.egui_ctx.style_mut(|style| {
            style.interaction.selectable_labels = false;
        });

        Self {
            main_window: MainWindow::new(),
        }
    }

    fn setup_fonts_static(cc: &eframe::CreationContext<'_>) {
        let mut fonts = egui::FontDefinitions::default();

        // Segoe UI, mdl2
        let extra_fonts: Vec<(&str, &[u8])> = vec![
            ("segoeui", include_bytes!("../assets/fonts/segoeui.ttf")),
            ("segmdl2", include_bytes!("../assets/fonts/segmdl2.ttf")),
        ];
        for (font_name, font_data) in extra_fonts {
            fonts.font_data.insert(
                font_name.to_owned(),
                Arc::new(egui::FontData::from_static(font_data)),
            );

            fonts
                .families
                .get_mut(&egui::FontFamily::Proportional)
                .unwrap()
                .insert(0, font_name.to_owned());

            fonts
                .families
                .get_mut(&egui::FontFamily::Monospace)
                .unwrap()
                .insert(0, font_name.to_owned());
        }

        cc.egui_ctx.set_fonts(fonts);
    }
}

// App render loop
impl eframe::App for CrynApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Render main window
        self.main_window.render(ctx);
    }
}
