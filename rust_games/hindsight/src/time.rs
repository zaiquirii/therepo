pub struct FixedTimeLoop {
    accumulated_time: f32,
    delta_time: f32,
}

impl FixedTimeLoop {
    pub fn new(delta_time: f32) -> Self {
        Self {
            accumulated_time: 0.0,
            delta_time,
        }
    }

    pub fn accumulate(&mut self, time: f32) {
        self.accumulated_time += time;
    }

    pub fn step(&mut self) -> bool {
        if self.accumulated_time >= self.delta_time {
            self.accumulated_time -= self.delta_time;
            true
        } else {
            false
        }
    }
}
