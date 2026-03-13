use eframe::egui;

pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "digital_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../../assets/PixelMplus12-Regular.ttf")),
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
