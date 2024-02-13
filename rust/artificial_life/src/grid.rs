use std::fmt::Debug;
use std::thread::current;
use macroquad::math::{Rect, UVec2, Vec2};

type Bucket = usize;

pub struct Grid<T> {
    elements: Vec<(Bucket, T)>,
    offsets: Vec<usize>,
    bounds: Rect,
    pub dimens: UVec2,
    pub origin: Vec2,
    pub cell_size: Vec2,
    finalized: bool,
}

impl<T: Debug> Grid<T> {
    pub fn new(capacity: usize, bounds: Rect, dimens: UVec2) -> Self {
        Self {
            finalized: false,
            elements: Vec::with_capacity(capacity),
            offsets: vec![0; (dimens.y * dimens.x + 1) as usize],
            bounds,
            dimens,
            origin: bounds.point(),
            cell_size: Vec2::new(
                bounds.w / (dimens.x as f32),
                bounds.h / (dimens.y as f32),
            ),
        }
    }

    pub fn reset(&mut self) {
        self.elements.clear();
        self.finalized = false;
    }

    pub fn insert(&mut self, pos: Vec2, elem: T) {
        let p = pos - self.origin;
        let x = (p.x / self.cell_size.x) as usize;
        let y = (p.y / self.cell_size.y) as usize;
        let bucket = y * self.dimens.x as usize + x;
        self.elements.push((bucket.clamp(0, self.elements.len()), elem));
    }

    pub fn finalize(&mut self) {
        self.finalized = true;
        self.elements.sort_by_key(|e| e.0);

        let mut next_offset = 0;
        for (i, (bucket, _)) in self.elements.iter().enumerate() {
            while *bucket >= next_offset && *bucket < self.offsets.len() {
                self.offsets[next_offset] = i;
                next_offset += 1;
            }
        }
        let len = self.offsets.len();
        self.offsets[len - 1] = self.elements.len();
    }

    pub fn scan(&self, pos: Vec2, range: f32) -> GridIter<'_, T> {
        let start_x = ((pos.x - range) / self.cell_size.x).clamp(0.0, (self.dimens.x - 1) as f32) as usize;
        let start_y = ((pos.y - range) / self.cell_size.y).clamp(0.0, (self.dimens.y - 1) as f32) as usize;
        let end_x = ((pos.x + range) / self.cell_size.x).clamp(0.0, (self.dimens.x - 1) as f32) as usize;
        let end_y = ((pos.y + range) / self.cell_size.y).clamp(0.0, (self.dimens.y - 1) as f32) as usize;

        let bucket = start_x + start_y * self.dimens.x as usize;
        let s = self.offsets[bucket];
        let g = GridIter {
            grid: self,
            start: (start_x, start_y),
            dims: (end_x - start_x + 1, end_y - start_y + 1),
            index: 0,
        };
        g
    }
}

pub struct GridIter<'a, T> {
    grid: &'a Grid<T>,
    start: (usize, usize),
    dims: (usize, usize),
    index: usize,
}

impl<'a> Iterator for GridIter<'a, usize> {
    type Item = &'a [(Bucket, usize)];

    fn next(&mut self) -> Option<Self::Item> {
        let block_x = self.index % self.dims.0;
        let block_y = self.index / self.dims.0;


        if block_y >= self.dims.1 {
            return None;
        }

        let bucket = (self.start.1 + block_y) * self.grid.dimens.x as usize + (self.start.0 + block_x);
        let start_index = self.grid.offsets[bucket];
        let end_index = self.grid.offsets[bucket + 1];
        let ret = &self.grid.elements[start_index..end_index];
        self.index += 1;
        Some(ret)
    }
}

