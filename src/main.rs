#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod components;
mod timer;

use eframe::egui; // egui のフレームワーク

// アプリの実行結果を返すメイン関数
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 150.0]) //
            .with_decorations(true) //
            .with_transparent(true) //
            .with_always_on_top() //
            .with_icon(load_icon()),
        ..Default::default() //
    };

    // メインループ実行
    eframe::run_native(
        "RTM - Rust Timer",
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx); //

            let app = app::RtmApp::new(cc);
            Ok(Box::new(app) as Box<dyn eframe::App>)
        }),
    )
}

// egui のデフォルトフォントを設定する関数
fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // コンパイル時に指定したファイルをバイナリデータとして .exe の中に埋め込む
    fonts.font_data.insert(
        "digital_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/PixelMplus12-Regular.ttf")),
    );

    // 文字化け（ガタガタ）防止
    fonts
        .families
        .entry(egui::FontFamily::Proportional) // 可変幅
        .or_default()
        .insert(0, "digital_font".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Monospace) // 等幅
        .or_default()
        .insert(0, "digital_font".to_owned());

    // 設定したフォントデータを egui コンテキスト（アプリ全体）に反映
    ctx.set_fonts(fonts);
}

// icon データ設定
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
