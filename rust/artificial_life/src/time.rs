pub struct FixedTimeLoop {
    accumulated_time: f32,
}

impl FixedTimeLoop {
    pub fn new() -> Self {
        Self {
            accumulated_time: 0.0,
        }
    }

    pub fn accumulate(&mut self, time: f32) {
        self.accumulated_time += time;
    }

    pub fn tick(&mut self, frame_rate: f32) -> bool {
        if self.accumulated_time >= frame_rate {
            self.accumulated_time -= frame_rate;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        self.accumulated_time = 0.0;
    }
}


pub struct MovingAverage {
    index: usize,
    values: Vec<i32>,
}

impl MovingAverage {
    pub fn new(window: usize) -> Self {
        Self {
            index: 0,
            values: vec![0; window],
        }
    }

    pub fn update(&mut self, value: i32) {
        self.values[self.index] = value;
        self.index = (self.index + 1) % self.values.len();
    }

    pub fn avg(&self) -> i32 {
        self.values.iter().sum::<i32>() / self.values.len() as i32
    }
}