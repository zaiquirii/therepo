use std::collections::VecDeque;
use crate::hindsight::FrameId;

pub struct FrameCache<T: Default> {
    oldest_frame: u32,
    capacity: usize,
    buffer: VecDeque<T>,
}

impl<T: Default> FrameCache<T> {
    pub fn new(capacity: usize) -> Self {
        let mut buffer = VecDeque::with_capacity(capacity);
        for _ in 0..capacity {
            buffer.push_back(T::default())
        }
        Self {
            oldest_frame: 0,
            capacity,
            buffer,
        }
    }

    pub fn get_mut(&mut self, frame: FrameId) -> &mut T {
        assert!(frame.0 >= self.oldest_frame);
        assert!(frame.0 < self.oldest_frame + self.capacity as u32);
        let index = (frame.0 - self.oldest_frame) as usize;
        &mut self.buffer[index]
    }

    pub fn set(&mut self, frame: FrameId, value: T) {
        assert!(frame.0 >= self.oldest_frame);
        let index = (frame.0 - self.oldest_frame) as usize;
        if index > self.capacity {
            panic!("investigate the use here. should only ever go one frame further")
        } else if index == self.capacity {
            self.buffer.pop_front();
            self.buffer.push_back(value);
            self.oldest_frame += 1
        } else {
            self.buffer[index] = value
        }

        debug_assert!(self.capacity == self.buffer.capacity());
        debug_assert!(self.capacity == self.buffer.len());
    }
}