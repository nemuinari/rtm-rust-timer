use crate::components::logic::Timer;
use eframe::egui;

pub struct RtmApp {
    timer: Timer,
}

impl RtmApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self {
            timer: Timer::new(),
        }
    }
}

impl eframe::App for RtmApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut pending_command = None;

        // --- 入力判定 ---
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

                    // タイマー表示
                    let time_str = self.timer.format_elapsed();
                    crate::components::ui::timer_display(ui, time_str);

                    ui.add_space(20.0);

                    // ボタンの描画とアクション受け取り
                    if let Some(action) =
                        crate::components::ui::control_buttons(ui, self.timer.is_running)
                    {
                        match action {
                            crate::components::ui::TimerAction::Toggle => self.timer.toggle(),
                            crate::components::ui::TimerAction::Reset => self.timer.reset(),
                        }
                    }
                });
            });
        // --- コマンド実行 & 再描画要求 ---
        if let Some(cmd) = pending_command {
            ctx.send_viewport_cmd(cmd);
        }

        // タイマー稼働中のみ毎フレーム更新（CPU負荷を抑える）
        if self.timer.is_running {
            ctx.request_repaint();
        }
    }
}
