use crate::timer::Timer;
use chrono::Duration as ChronoDuration;
use eframe::egui;

pub struct RtmApp {
    timer: Timer,
}

impl RtmApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        _cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self {
            timer: Timer::new(),
        }
    }
}

impl eframe::App for RtmApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut pending_command = None;

        // ---  入力判定 ---
        ctx.input_mut(|i| {
            if i.consume_key(egui::Modifiers::NONE, egui::Key::S) {
                self.timer.toggle();
            }
            if i.consume_key(egui::Modifiers::NONE, egui::Key::R) {
                self.timer.reset();
            }
            if i.consume_key(egui::Modifiers::NONE, egui::Key::Space) {
                pending_command = Some(egui::ViewportCommand::Minimized(true));
            }
            if i.consume_key(egui::Modifiers::NONE, egui::Key::Escape) {
                pending_command = Some(egui::ViewportCommand::Close);
            }
        });

        // --- UI描画 ---
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(ctx.style().visuals.panel_fill))
            .show(ctx, |ui| {
                let resp = ui.interact(ui.max_rect(), ui.id().with("bg"), egui::Sense::drag());
                if resp.drag_started() {
                    pending_command = Some(egui::ViewportCommand::StartDrag);
                }

                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);

                    // タイマー
                    let elapsed = self.timer.get_elapsed();
                    let dur = ChronoDuration::from_std(elapsed)
                        .unwrap_or_else(|_| ChronoDuration::zero());
                    let time_str = format!(
                        "{:02}:{:02}:{:02}:{:02}",
                        dur.num_hours(),
                        dur.num_minutes() % 60,
                        dur.num_seconds() % 60,
                        (dur.num_milliseconds() / 10) % 100
                    );
                    ui.label(egui::RichText::new(time_str).size(45.0).monospace());

                    ui.add_space(20.0);

                    // ボタン
                    ui.horizontal(|ui| {
                        let total_w = 215.0;
                        ui.add_space((ui.available_width() - total_w) / 2.0);

                        let (label, color) = if self.timer.is_running {
                            ("STOP [S]", egui::Color32::from_rgb(200, 50, 50))
                        } else {
                            ("START [S]", egui::Color32::from_rgb(50, 100, 200))
                        };

                        if ui
                            .add_sized([100.0, 35.0], egui::Button::new(label).fill(color))
                            .clicked()
                        {
                            self.timer.toggle();
                        }
                        ui.add_space(15.0);
                        if ui
                            .add_sized(
                                [100.0, 35.0],
                                egui::Button::new("RESET [R]")
                                    .fill(egui::Color32::from_rgb(70, 80, 100)),
                            )
                            .clicked()
                        {
                            self.timer.reset();
                        }
                    });
                });
            });

        // ---  コマンド実行 ---
        if let Some(cmd) = pending_command {
            ctx.send_viewport_cmd(cmd);
        }

        if self.timer.is_running {
            ctx.request_repaint();
        }
    }
}
