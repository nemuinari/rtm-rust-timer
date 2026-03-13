use crate::components::icon;
use eframe::egui;

pub const DEFAULT_WIDTH: f32 = 300.0;
pub const DEFAULT_HEIGHT: f32 = 150.0;

/// ウィンドウの初期設定
pub fn get_native_options() -> eframe::NativeOptions {
    let window_size = egui::vec2(DEFAULT_WIDTH, DEFAULT_HEIGHT);

    eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(window_size)
            .with_min_inner_size(window_size)
            .with_max_inner_size(window_size)
            .with_maximize_button(true)
            .with_resizable(true)
            .with_decorations(true)
            .with_always_on_top()
            .with_icon(icon::load_icon()),
        ..Default::default()
    }
}
