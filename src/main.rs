#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod components;
mod timer;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 150.0])
            .with_decorations(true)
            .with_transparent(true)
            .with_always_on_top()
            .with_icon(load_icon()),
        ..Default::default()
    };

    eframe::run_native(
        "RTM - Rust Timer",
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            let app = app::RtmApp::new(cc);
            Ok(Box::new(app) as Box<dyn eframe::App>)
        }),
    )
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // フォントデータを埋め込み
    fonts.font_data.insert(
        "digital_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/PixelMplus12-Regular.ttf")),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "digital_font".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "digital_font".to_owned());

    ctx.set_fonts(fonts);
}

fn load_icon() -> std::sync::Arc<egui::IconData> {
    let icon_bytes = include_bytes!("../assets/icon.png");
    let image = image::load_from_memory(icon_bytes)
        .expect("Failed to load icon")
        .into_rgba8();

    let (width, height) = image.dimensions();
    std::sync::Arc::new(egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    })
}
