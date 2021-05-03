use std::{f64::NAN, time::Duration};

pub struct FrameCounter {
    fps: f64,
    tacc: Duration,
    /// Exponential decay factor of previous values.
    /// Deafult value is `0.1`.
    pub decay: f64,
    /// logging period
    /// Deafult is 2 seconds.
    pub log_period: Option<Duration>,
}

impl FrameCounter {
    pub fn new() -> Self {
        Self {
            fps: NAN,
            tacc: Duration::from_secs(0),
            decay: 1e-1,
            log_period: None,
        }
    }

    pub fn new_with_log(log_period: Duration) -> Self {
        Self {
            fps: NAN,
            tacc: Duration::from_secs(0),
            decay: 1e-1,
            log_period: Some(log_period),
        }
    }

    /// Frames per second.
    pub fn fps(&self) -> f64 {
        self.fps
    }

    pub fn step_frame(&mut self, time: Duration, n_passes: usize) {
        let cfps = (n_passes as f64) / ((time.as_secs() as f64) + 1e-6 * (time.as_micros() as f64));
        if self.fps.is_nan() {
            self.fps = cfps;
        } else {
            self.fps = (1.0 - self.decay) * self.fps + self.decay * cfps;
        }

        self.tacc += time;
        if let Some(log_period) = self.log_period {
            if self.tacc > log_period {
                println!("FPS: {:.2}", self.fps);
                self.tacc = Duration::from_secs(0);
            }
        }
    }
}
