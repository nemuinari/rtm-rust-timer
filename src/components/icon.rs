use eframe::egui;
use std::sync::Arc;

pub fn load_icon() -> Arc<egui::IconData> {
    let icon_bytes = include_bytes!("../../assets/icon.png");
    let image = image::load_from_memory(icon_bytes)
        .expect("Failed to load icon")
        .into_rgba8();
    let (width, height) = image.dimensions();
    Arc::new(egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    })
}
