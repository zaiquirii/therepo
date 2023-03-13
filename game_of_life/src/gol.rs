use std::cell;

type Generation = Vec<bool>;

pub struct Simulation {
    width: usize,
    height: usize,
    visible_buffer: Generation,
    simulation_buffer: Generation,
}

impl Simulation {
    pub fn new(width: usize, height: usize) -> Self {
        Simulation {
            width,
            height,
            visible_buffer: vec![false; width * height],
            simulation_buffer: vec![false; width * height],
        }
    }

    pub fn toggle(&mut self, x: usize, y: usize) {
        eprintln!("width: {}, height: {}", self.width, self.height);
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);

        let index = y * self.width + x;
        self.visible_buffer[index] = !self.visible_buffer[index];
    }

    pub fn step(&mut self) {
        for i in 0..self.visible_buffer.len() {
            let cell_alive = self.visible_buffer[i];
            let neighbors = self.neighbor_count(i);

            if neighbors < 2 || neighbors > 3 {
                self.simulation_buffer[i] = false;
            } else if neighbors == 3 {
                self.simulation_buffer[i] = true;
                // self.simulation_buffer[i] = cell_alive;
            } else {
                self.simulation_buffer[i] = cell_alive;
            }
        }
        self.swap()
    }

    pub fn swap(&mut self) {
        std::mem::swap(&mut self.simulation_buffer, &mut self.visible_buffer);
    }

    fn neighbor_count(&self, index: usize) -> u8 {
        let x = (index % self.width) as i32;
        let y = (index / self.width) as i32;

        let mut count = 0;
        for offset_y in -1..2 {
            for offset_x in -1..2 {
                if offset_x == 0 && offset_y == 0 {
                    continue;
                }

                let target_x = x + offset_x;
                let target_y = y + offset_y;

                if target_x >= 0
                    && (target_x as usize) < self.width
                    && target_y >= 0
                    && (target_y as usize) < self.height
                    && self.visible_buffer[(target_x as usize) + (target_y as usize) * self.width]
                {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let rgba = if self.visible_buffer[i] {
                [255, 255, 255, 255]
            } else {
                [100, 0, 0, 255]
            };
            pixel.copy_from_slice(&rgba);
        }
    }
}
