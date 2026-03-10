#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod timer;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let width = 300.0;
    let height = 150.0;
    let window_size = egui::vec2(width, height);
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(window_size)
            .with_min_inner_size(window_size) // 最小を固定
            .with_max_inner_size(window_size) // 最大を固定
            .with_maximize_button(true)
            .with_resizable(true)
            .with_decorations(true)
            .with_always_on_top()
            .with_icon(load_icon()),
        ..Default::default()
    };

    eframe::run_native(
        "RTM - Rust Timer",
        options,
        Box::new(move |cc| {
            setup_custom_fonts(&cc.egui_ctx);
            // app 呼び出し
            Ok(Box::new(app::RtmApp::new(cc)) as Box<dyn eframe::App>)
        }),
    )
}

fn setup_custom_fonts(ctx: &egui::Context) {
    // Fonts 設定
    let mut fonts = egui::FontDefinitions::default();
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
    // Icons 設定
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
