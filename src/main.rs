mod app;
mod theme;
mod tree;

use app::UnfurlApp;

/// native entry
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    use eframe::egui;

    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Unfurl")
            .with_inner_size([1024.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Unfurl",
        native_options,
        Box::new(|cc| {
            theme::apply(&cc.egui_ctx);
            Ok(Box::new(UnfurlApp::default()))
        }),
    )
}

/// web entry
#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    eframe::WebLogger::init(log::LevelFilter::Debug).ok(); // panics to browser

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Canvas element not found")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("Not a canvas element");

        eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| {
                    theme::apply(&cc.egui_ctx);
                    Ok(Box::new(UnfurlApp::default()))
                }),
            )
            .await
            .expect("Failed to start eframe");
    });
}
