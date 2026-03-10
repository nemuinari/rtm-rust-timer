use std::time::{Duration, Instant}; // 標準モジュール

pub struct Timer {
    pub is_running: bool,           // タイマーが現在動いているかどうかのフラグ
    elapsed_before_start: Duration, // 合計時間
    start_instant: Option<Instant>, // 「START」を押した瞬間の時刻（止まっている時は None）
}

impl Timer {
    pub fn new() -> Self {
        Self {
            is_running: false,
            elapsed_before_start: Duration::ZERO,
            start_instant: None,
        }
    }

    // 開始と停止を交互に切り替えるトグル
    pub fn toggle(&mut self) {
        if self.is_running {
            self.stop();
        } else {
            self.start();
        }
    }

    // 計測を開始
    fn start(&mut self) {
        if !self.is_running {
            self.start_instant = Some(Instant::now());
            self.is_running = true;
        }
    }

    // 計測を一時停止
    fn stop(&mut self) {
        if self.is_running {
            if let Some(start) = self.start_instant {
                self.elapsed_before_start += start.elapsed();
            }
            self.start_instant = None;
            self.is_running = false;
        }
    }

    // リセット、初期化
    pub fn reset(&mut self) {
        self.is_running = false;
        self.elapsed_before_start = Duration::ZERO;
        self.start_instant = None;
    }

    // 計算結果を出力
    pub fn get_elapsed(&self) -> Duration {
        match self.start_instant {
            Some(start) => self.elapsed_before_start + start.elapsed(),
            None => self.elapsed_before_start,
        }
    }
}
