use macroquad::prelude::*;

pub struct Game {
    field: PlayingField,
    next_block: Option<Block>,
}

impl Game {
    pub fn new(field_size: UVec2) -> Self {
        let mut g = Game {
            field: PlayingField::new(field_size),
            next_block: None,
        };
        g
    }

    pub fn render(&self) {
        self.field.render()
    }
}

pub struct PlayingField {
    size: UVec2,
    cells: Vec<Cell>,
    active_block: Option<Block>,
}

impl PlayingField {
    fn new(field_size: UVec2) -> Self {
        PlayingField {
            size: field_size,
            cells: vec![Cell::Empty; (field_size.x * field_size.y) as usize],
            active_block: None,
        }
    }

    fn cell_index(&self, x: usize, y: usize) -> usize {
        y * self.size.x as usize + x
    }

    fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        let i = self.cell_index(x, y);
        self.cells[i] = cell
    }

    fn set_active_block(&mut self, block: Option<Block>) {
        self.active_block = block
    }

    fn render(&self) {
        let cell_size = 20.0;
        draw_rectangle(
            0.0, 0.0,
            cell_size * (self.size.x + 1) as f32,
            cell_size * (self.size.y + 1) as f32,
            GRAY,
        );
        draw_rectangle(
            0.0, 0.0,
            cell_size * self.size.x as f32,
            cell_size * self.size.y as f32,
            BLACK
        );

        // Draw Active Block
        match &self.active_block {
            Some(b) => {
                render_block(
                    b.pos.x as f32 * cell_size,
                    b.pos.y as f32 * cell_size,
                    cell_size,
                    b
                )
            }
            _ => {}
        }

        // Draw existing field
        for y in 0..self.size.y as usize {
            for x in 0..self.size.x as usize {
                render_cell(
                    x as f32 * cell_size,
                    y as f32 * cell_size,
                    cell_size,
                    self.cells[x + y * self.size.x as usize],
                )
            }
        }
    }
}

fn render_block(x: f32, y: f32, cell_size: f32, block: &Block) {
    let offsets = [(0, 0), (1, 0), (0, 1), (1, 1)];

}

fn render_cell(x: f32, y: f32, cell_size: f32, cell: Cell) {
    match cell {
        Cell::Empty => {}
        Cell::Filled(c) => {
            draw_rectangle(x, y, cell_size, cell_size, c)
        }
    }
}

#[derive(Clone, Copy)]
pub enum Cell {
    Empty,
    Filled(Color),
}

pub struct Block {
    pos: IVec2,
    shape: Tetromino,
    rot: Rotation,
}

pub enum Rotation {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

pub enum Tetromino {
    I,
    J,
    L,
    O,
    T,
    S,
    Z,
}