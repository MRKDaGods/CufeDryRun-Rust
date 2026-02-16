use crate::services::CourseManager;
use crate::utils;
use crate::windows::MainWindow;
use std::sync::Arc;

pub struct CrynApp {
    /* Windows */
    main_window: MainWindow,

    /* Whatever */
    course_manager: CourseManager,
}

impl CrynApp {
    /// App ctor
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        utils::log("Cryn started");

        // Configure theme
        //let mut visuals = egui::Visuals::dark();
        //visuals.override_text_color = Some(egui::Color32::WHITE);
        //cc.egui_ctx.set_visuals(visuals);

        // Fonts
        Self::setup_fonts(cc);

        // Turn off text selection
        cc.egui_ctx.style_mut(|style| {
            style.interaction.selectable_labels = false;
        });

        Self {
            main_window: MainWindow::new(),
            course_manager: Self::initialize_course_manager(),
        }
    }

    fn setup_fonts(cc: &eframe::CreationContext<'_>) {
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

    fn initialize_course_manager() -> CourseManager {
        let mut course_manager = CourseManager::new();
        let data = include_str!("../assets/data/sample_courses.txt");
        //course_manager.parse_courses(data);

        println!("Courses: {:?}", course_manager.course_records);

        course_manager
    }
}

// App render loop
impl eframe::App for CrynApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Render main window
        self.main_window.render(ctx);
    }
}
