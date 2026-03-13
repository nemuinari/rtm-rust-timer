use eframe::egui;

// どのボタンが押されたかを表す
pub enum TimerAction {
    Toggle,
    Reset,
}

pub fn timer_display(ui: &mut egui::Ui, time_str: String) {
    ui.label(egui::RichText::new(time_str).size(45.0).monospace());
}

pub fn control_buttons(ui: &mut egui::Ui, is_running: bool) -> Option<TimerAction> {
    let mut action = None;

    ui.horizontal(|ui| {
        let total_w = 215.0;
        ui.add_space((ui.available_width() - total_w) / 2.0);

        /* --- btn colors --- */
        let (label, color) = if is_running {
            ("STOP [S]", egui::Color32::from_rgb(200, 50, 50))
        } else {
            ("START [S]", egui::Color32::from_rgb(50, 100, 200))
        };

        if ui
            .add_sized([100.0, 35.0], egui::Button::new(label).fill(color))
            .clicked()
        {
            action = Some(TimerAction::Toggle);
        }

        ui.add_space(15.0);

        if ui
            .add_sized(
                [100.0, 35.0],
                egui::Button::new("RESET [R]").fill(egui::Color32::from_rgb(70, 80, 100)),
            )
            .clicked()
        {
            action = Some(TimerAction::Reset);
        }
    });

    action // 押されたボタンを返す
}
