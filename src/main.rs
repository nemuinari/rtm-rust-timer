#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod components;

use components::{app, font, frame};

fn main() -> eframe::Result<()> {
    let options = frame::get_native_options();

    eframe::run_native(
        "RTM - Rust Timer",
        options,
        Box::new(move |cc| {
            font::setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(app::RtmApp::new(cc)) as Box<dyn eframe::App>)
        }),
    )
}
