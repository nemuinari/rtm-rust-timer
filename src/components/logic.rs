use chrono::Duration as ChronoDuration;
use std::time::{Duration, Instant};

pub struct Timer {
    pub is_running: bool,
    elapsed_before_start: Duration,
    start_instant: Option<Instant>,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            is_running: false,
            elapsed_before_start: Duration::ZERO,
            start_instant: None,
        }
    }

    pub fn toggle(&mut self) {
        if self.is_running {
            self.stop();
        } else {
            self.start();
        }
    }

    fn start(&mut self) {
        if !self.is_running {
            self.start_instant = Some(Instant::now());
            self.is_running = true;
        }
    }

    fn stop(&mut self) {
        if self.is_running {
            if let Some(start) = self.start_instant {
                self.elapsed_before_start += start.elapsed();
            }
            self.start_instant = None;
            self.is_running = false;
        }
    }

    pub fn reset(&mut self) {
        self.is_running = false;
        self.elapsed_before_start = Duration::ZERO;
        self.start_instant = None;
    }

    pub fn get_elapsed(&self) -> Duration {
        match self.start_instant {
            Some(start) => self.elapsed_before_start + start.elapsed(),
            None => self.elapsed_before_start,
        }
    }
    /// 現在の経過時間を "HH:MM:SS:CC" 形式の文字列で返す
    pub fn format_elapsed(&self) -> String {
        let elapsed = self.get_elapsed();
        let dur = ChronoDuration::from_std(elapsed).unwrap_or_else(|_| ChronoDuration::zero());

        let centiseconds = (dur.num_milliseconds() / 10) % 100;

        format!(
            "{:02}:{:02}:{:02}:{:02}",
            dur.num_hours(),
            dur.num_minutes() % 60,
            dur.num_seconds() % 60,
            centiseconds
        )
    }
}
