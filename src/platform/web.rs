use cryn_rs::CrynApp;
use web_sys::HtmlCanvasElement;

const CANVAS_ID: &str = "container-canv";

pub fn run() {
    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let canvas = get_canvas().expect("Failed to get canvas");

        eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(CrynApp::new(cc)))),
            )
            .await
            .expect("Failed to start web app");
    });
}

fn get_canvas() -> Option<HtmlCanvasElement> {
    use eframe::wasm_bindgen::JsCast;
    use web_sys::window;

    let canvas = window()?
        .document()?
        .get_element_by_id(CANVAS_ID)?
        .dyn_into::<HtmlCanvasElement>()
        .ok()?;

    Some(canvas)
}
