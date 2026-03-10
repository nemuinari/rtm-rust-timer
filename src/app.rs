use crate::timer::Timer;
use chrono::Duration as ChronoDuration; // 秒数を "00:00" 形式に整形
use eframe::egui;

pub struct RtmApp {
    timer: Timer, // timer.rs: タイマーロジック
}

impl RtmApp {
    /// アプリケーションの初期化
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // ダークモードを適用
        _cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self {
            timer: Timer::new(),
        }
    }

    // --- キーバインド ---
    fn handle_inputs(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            // [S] キーで開始/停止
            if i.key_pressed(egui::Key::S) {
                self.timer.toggle();
            }
            // [R] キーでリセット
            if i.key_pressed(egui::Key::R) {
                self.timer.reset();
            }
            // [Space] キーで最小化
            if i.key_pressed(egui::Key::Space) {
                ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
            }
            // [ESC] キーで終了
            if i.key_pressed(egui::Key::Escape) {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }

    // --- UI 管理 ---
    fn render_ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // ウィンドウ全体のドラッグ移動を可能にする
            self.setup_drag_area(ctx, ui);

            ui.vertical_centered(|ui| {
                ui.add_space(15.0);

                // タイマー表示部分
                self.draw_timer_text(ui);

                ui.add_space(15.0);

                // 操作ボタン部分
                self.draw_buttons(ui);
            });
        });
    }

    // --- ドラッグエリアの設定 ---
    fn setup_drag_area(&self, ctx: &egui::Context, ui: &mut egui::Ui) {
        if ui
            .interact(ui.max_rect(), ui.id(), egui::Sense::drag())
            .dragged()
        {
            ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
        }
    }

    // ---  タイマーテキストの描画ロジック ---
    fn draw_timer_text(&self, ui: &mut egui::Ui) {
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
    }

    // --- ボタン配置の描画ロジック ---
    fn draw_buttons(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 15.0;

            let btn_width = 110.0;
            let btn_height = 35.0;

            // ボタンを中央に寄せるためのスペース計算
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
            let reset_btn = egui::Button::new(egui::RichText::new("RESET [R]").size(16.0).strong())
                .fill(egui::Color32::from_rgb(70, 80, 100));

            if ui.add_sized([btn_width, btn_height], reset_btn).clicked() {
                self.timer.reset();
            }
        });
    }
}

impl eframe::App for RtmApp {
    /// メインループ（毎フレーム実行）
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 入力を受け取る
        self.handle_inputs(ctx);

        // 画面を描画する
        self.render_ui(ctx);

        // タイマー稼働中のみ再描画を要求してアニメーションさせる
        if self.timer.is_running {
            ctx.request_repaint();
        }
    }
}
