use std::time::Duration;


pub struct FrameCounter {
    fps: f64,
    tacc: Duration,
    /// Exponential decay factor of previous values.
    /// Deafult value is `0.1`.
    pub decay: f64,
    /// logging period
    /// Deafult is 2 seconds.
    pub log_period: Duration,
}

impl FrameCounter {
    pub fn new() -> Self {
        Self {
            fps: 0.0, tacc: Duration::from_secs(0),
            decay: 1e-1, log_period: Duration::from_secs(2),
        }
    }

    /// Frames per second.
    pub fn fps(&self) -> f64 {
        self.fps
    }

    pub fn step_frame(&mut self, time: Duration, n_passes: usize) {
        let cfps = (n_passes as f64)/((time.as_secs() as f64) + 1e-6*(time.as_micros() as f64));
        self.fps = (1.0 - self.decay)*self.fps + self.decay*cfps;

        self.tacc += time;
        if self.tacc > self.log_period {
            println!("FPS: {:.2}", self.fps);
            self.tacc = Duration::from_secs(0);
        }
    }
}


