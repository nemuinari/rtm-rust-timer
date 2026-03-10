use crate::timer::Timer;
use chrono::Duration as ChronoDuration; // 標準のDurationを、00:00 形式で整形がしやすいChrono型として扱う
use eframe::egui;

/// アプリケーションの状態を保持する構造体
pub struct RtmApp {
    timer: Timer, // タイマーロジック（timer.rs）を保持
}

impl RtmApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // ダークモードに設定
        _cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self {
            timer: Timer::new(), // 新規タイマーインスタンスを作成
        }
    }

    /// キーボード入力のハンドリング
    fn handle_inputs(&mut self, ctx: &egui::Context) {
        // コマンド送信フラグ（ctx.inputの借用中に直接ウィンドウ操作コマンドを送ると不安定になるのを防ぐ）
        let mut should_minimize = false;
        let mut should_close = false;

        ctx.input(|i| {
            // [S] キー：開始/停止の切り替え
            if i.key_pressed(egui::Key::S) {
                self.timer.toggle();
            }
            // [R] キー：タイマーリセット
            if i.key_pressed(egui::Key::R) {
                self.timer.reset();
            }
            // [Space] キー：最小化フラグを立てる
            if i.key_pressed(egui::Key::Space) {
                should_minimize = true;
            }
            // [Escape] キー：終了フラグを立てる
            if i.key_pressed(egui::Key::Escape) {
                should_close = true;
            }
        });

        // 借用が終わった後で、OS（ウィンドウマネージャ）に命令を送信
        if should_minimize {
            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
        }
        if should_close {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }

    /// メインパネルの描画
    fn render_ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // 背景部分でのドラッグ移動を可能にする設定
            // 画面全体の矩形（max_rect）に対してドラッグ（Sense::drag）を検知させる
            let drag_response = ui.interact(ui.max_rect(), ui.id(), egui::Sense::drag());
            if drag_response.dragged() {
                ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
            }

            // UI要素を垂直方向に中央揃えで配置
            ui.vertical_centered(|ui| {
                ui.add_space(15.0); // 上部余白
                self.draw_timer_text(ui); // タイマー数字の描画
                ui.add_space(15.0); // 中間余白
                self.draw_buttons(ui); // 操作ボタンの描画
            });
        });
    }

    /// タイマーテキスト（00:00:00:00）の描画
    fn draw_timer_text(&self, ui: &mut egui::Ui) {
        // timer.rsから現在の経過時間を取得
        // 標準のDurationをChronoのDurationに変換
        let elapsed = self.timer.get_elapsed();
        let duration = ChronoDuration::from_std(elapsed).unwrap_or(ChronoDuration::zero());

        // 表示形式を整える
        let time_str = format!(
            "{:02}:{:02}:{:02}:{:02}",
            duration.num_hours(),
            duration.num_minutes() % 60,
            duration.num_seconds() % 60,
            (duration.num_milliseconds() / 10) % 100
        );

        // タイマー描画
        ui.label(egui::RichText::new(time_str).size(40.0).monospace());
    }

    /// 操作ボタンの描画
    fn draw_buttons(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 15.0; // ボタン間の水平余白
            let btn_width = 110.0;
            let btn_height = 35.0;

            // 中央配置にするためのスペースを計算
            let total_width = (btn_width * 2.0) + ui.spacing().item_spacing.x;
            ui.add_space((ui.available_width() - total_width) / 2.0);

            // タイマーの状態によってボタンのラベルと色を動的に変更
            let (btn_label, btn_color) = if self.timer.is_running {
                ("STOP [S]", egui::Color32::from_rgb(200, 50, 50)) // 停止時は赤系
            } else {
                ("START [S]", egui::Color32::from_rgb(50, 100, 200)) // 開始時は青系
            };

            // START / STOP ボタンの追加とクリック判定
            if ui
                .add_sized(
                    [btn_width, btn_height],
                    egui::Button::new(btn_label).fill(btn_color),
                )
                .clicked()
            {
                self.timer.toggle();
            }

            // RESET ボタンの追加とクリック判定
            if ui
                .add_sized(
                    [btn_width, btn_height],
                    egui::Button::new("RESET [R]").fill(egui::Color32::from_rgb(70, 80, 100)),
                )
                .clicked()
            {
                self.timer.reset();
            }
        });
    }
}

/// メインループ
impl eframe::App for RtmApp {
    /// 毎フレーム呼ばれる更新・描画関数
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_inputs(ctx); // ユーザーの入力をチェック
        self.render_ui(ctx); // 画面の状態を描画

        // 描画制御
        if self.timer.is_running {
            ctx.request_repaint();
        }
    }
}
