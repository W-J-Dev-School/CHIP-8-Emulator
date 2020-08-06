use std::time::{Duration, Instant};

pub struct Clock {
    period: Duration,
    offset: Instant,
}

impl Clock {
    pub fn new(freq: u16) -> Self {
        Self {
            period: Duration::from_nanos(1_000_000_000 / freq as u64),
            offset: Instant::now()
        }
    }

    pub fn tick(&mut self) -> bool {
        if self.offset.elapsed() >= self.period {
            self.offset += self.period;
            true
        } else {
            false
        }
    }
}