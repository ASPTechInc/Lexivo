#![warn(clippy::all, rust_2018_idioms)]

// Lexivo Shared Library
//
// This module contains the core application logic and platform-specific entry points.

pub mod app;
pub mod changelog;
pub mod config;
pub mod platform_feedback;
pub mod screens;
pub mod theme;
pub mod types;
pub mod utils;
pub use app::LexivoApp;

/// The main entry point for the Android version of the Lexivo application.
/// This function is called by the `android_activity` crate to initialise the app on Android.
#[cfg(target_os = "android")]
#[allow(unsafe_code)]
#[unsafe(no_mangle)]
fn android_main(app: android_activity::AndroidApp) {
    // Initialise logging for Android, so we can see output in logcat.
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    let mut options = eframe::NativeOptions::default();
    // Provide the Android app handle to eframe.
    options.android_app = Some(app);

    // Run the application natively using eframe.
    let _ = eframe::run_native(
        config::AppConfig::APP_NAME,
        options,
        Box::new(|cc| Ok(Box::new(app::LexivoApp::new(cc)))),
    );
}
