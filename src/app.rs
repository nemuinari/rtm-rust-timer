use crate::timer::Timer;
use chrono::Duration as ChronoDuration;
use eframe::egui;

pub struct RtmApp {
    timer: Timer,
}

impl RtmApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // ダークモードを強制適用
        _cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self {
            timer: Timer::new(),
        }
    }
}

impl eframe::App for RtmApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- キーバインド処理 ---
        if ctx.input(|i| i.key_pressed(egui::Key::S)) {
            self.timer.toggle();
        }
        if ctx.input(|i| i.key_pressed(egui::Key::R)) {
            self.timer.reset();
        }
        if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
        }
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        // --- UI描画 ---
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui
                .interact(ui.max_rect(), ui.id(), egui::Sense::drag())
                .dragged()
            {
                ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
            }

            ui.vertical_centered(|ui| {
                ui.add_space(15.0);

                // タイマー表示 (00:00:00:00)
                let elapsed = self.timer.get_elapsed();
                let duration = ChronoDuration::from_std(elapsed).unwrap_or(ChronoDuration::zero());

                let time_str = format!(
                    "{:02}:{:02}:{:02}:{:02}",
                    duration.num_hours(),
                    duration.num_minutes() % 60,
                    duration.num_seconds() % 60,
                    (duration.num_milliseconds() / 10) % 100
                );

                ui.label(egui::RichText::new(time_str).size(40.0).monospace());

                ui.add_space(15.0);

                // ボタンエリア
                ui.horizontal(|ui| {
                    // ボタン同士の間隔
                    ui.spacing_mut().item_spacing.x = 15.0;

                    let btn_width = 110.0;
                    let btn_height = 35.0;
                    let total_width = (btn_width * 2.0) + ui.spacing().item_spacing.x;
                    ui.add_space((ui.available_width() - total_width) / 2.0);

                    // START / STOP ボタン
                    let (btn_label, btn_color) = if self.timer.is_running {
                        ("STOP [S]", egui::Color32::from_rgb(200, 50, 50))
                    } else {
                        ("START [S]", egui::Color32::from_rgb(50, 100, 200))
                    };

                    let start_stop_btn =
                        egui::Button::new(egui::RichText::new(btn_label).size(16.0).strong())
                            .fill(btn_color);

                    if ui
                        .add_sized([btn_width, btn_height], start_stop_btn)
                        .clicked()
                    {
                        self.timer.toggle();
                    }

                    // RESET ボタン
                    let reset_btn =
                        egui::Button::new(egui::RichText::new("RESET [R]").size(16.0).strong())
                            .fill(egui::Color32::from_rgb(70, 80, 100));

                    if ui.add_sized([btn_width, btn_height], reset_btn).clicked() {
                        self.timer.reset();
                    }
                });
            });
        });

        if self.timer.is_running {
            ctx.request_repaint();
        }
    }
}
