#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// Lexivo Native and Web Entry Point
//
// This file handles the platform-specific initialization for Desktop and Web builds.

#[cfg(not(target_arch = "wasm32"))]
use lexivo::config::AppConfig;

// When compiling natively (Linux, Windows, macOS):
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        AppConfig::APP_NAME,
        native_options,
        Box::new(|cc| Ok(Box::new(lexivo::LexivoApp::new(cc)))),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    use wasm_bindgen::JsCast as _;

    // Redirect `log` messages to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let runner = eframe::WebRunner::new();
        let canvas = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("the_canvas_id"))
            .and_then(|e| e.dyn_into::<web_sys::HtmlCanvasElement>().ok())
            .expect("failed to find canvas");

        runner
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(lexivo::LexivoApp::new(cc)))),
            )
            .await
            .expect("failed to start eframe");
    });
}
