use bevy::prelude::*;

pub struct Cell {}

pub struct Grid<T: Default + Copy> {
    width: usize,
    height: usize,
    cells: Vec<T>,
}

impl<T: Default + Copy> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        let mut cells = Vec::with_capacity(size);
        for _ in 0..size {
            cells.push(T::default());
        }
        Grid {
            width,
            height,
            cells,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> T {
        self.cells[y * self.width + x]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.cells[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.cells[y * self.width + x] = value
    }
}
