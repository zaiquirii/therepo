use lazy_static::lazy_static;
use macroquad::prelude::*;
use macroquad::rand::rand;
use crate::game::field::FieldState::Falling;
use crate::game::field::TickResult::{BlockLocked, ClearingLines, GameOver, LinesCleared, Updated};
use crate::game::game::CONFIG;

pub enum TickResult {
    BlockLocked,
    ClearingLines,
    LinesCleared(usize),
    GameOver,
    Updated,
}

pub enum Rotation {
    Left,
    Right,
}

pub enum FieldState {
    Falling,
    ClearingLines {
        from: usize,
        to: usize,
        progress: f32,
    },
}

pub struct PlayingField {
    pub size: UVec2,
    pub cells: Vec<Cell>,
    pub active_block: Option<Tetromino>,
    pub clear_time: f32,
    pub state: FieldState,
}

impl PlayingField {
    pub fn new(field_size: UVec2) -> Self {
        PlayingField {
            size: field_size,
            cells: vec![Cell::Empty; (field_size.x * field_size.y) as usize],
            active_block: None,
            clear_time: 0.2,
            state: Falling,
        }
    }

    fn cell_index(&self, x: usize, y: usize) -> usize {
        y * self.size.x as usize + x
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        let i = self.cell_index(x, y);
        self.cells[i] = cell
    }

    pub fn set_active_block(&mut self, block: Option<Tetromino>) {
        self.active_block = block
    }

    pub fn tick(&mut self, delta: f32) -> TickResult {
        match self.state {
            Falling => {
                // Game over check
                if let Some(b) = self.active_block {
                    if !self.can_fit_block(&b) {
                        return GameOver;
                    }
                }

                // Move Block
                if !self.move_active_block(IVec2::Y) {
                    self.commit_active_block();

                    let lines_filled = self.check_lines();
                    if lines_filled > 0 {
                        return ClearingLines;
                    } else {
                        return BlockLocked;
                    }
                }
                return Updated;
            }
            FieldState::ClearingLines { from, to, progress } => {
                let new_progress = progress + delta;
                if new_progress > self.clear_time {
                    self.clear_lines(from, to);
                    self.state = Falling;
                    return LinesCleared(to - from);
                }

                self.state = FieldState::ClearingLines {
                    from,
                    to,
                    progress: new_progress,
                };
                return ClearingLines;
            }
        }
    }

    fn check_lines(&mut self) -> usize {
        let lines: Vec<_> = self.cells
            .chunks(self.size.x as usize)
            .enumerate()
            .filter(|(_, chunk)| {
                chunk.iter().all(|c| matches!(c, Cell::Filled(_)))
            })
            .map(|(i, _)| i)
            .collect();
        if lines.len() > 0 {
            self.state = FieldState::ClearingLines {
                from: lines[0],
                to: lines[lines.len() - 1],
                progress: 0.0,
            }
        }
        lines.len()
    }

    fn clear_lines(&mut self, from: usize, to: usize) {
        let from_index = from * self.size.x as usize;
        let to_index = (to + 1) * self.size.x as usize;
        let offset = to_index - from_index;

        for i in (0..from_index).rev() {
            self.cells[i + offset] = self.cells[i];
        }
        for i in 0..offset {
            self.cells[i] = Cell::Empty;
        }
    }

    pub fn rotate_active_block(&mut self, r: Rotation) -> bool {
        match &self.active_block {
            None => { false }
            Some(b) => {
                let mut wb = b.clone();
                wb.rot = match r {
                    Rotation::Left => b.rot.wrapping_sub(1),
                    Rotation::Right => b.rot + 1
                } % b.rotation_count();

                if self.can_fit_block(&wb) {
                    self.active_block = Some(wb);
                    return true;
                }
                return false;
            }
        }
    }

    fn can_fit_block(&self, b: &Tetromino) -> bool {
        let pos = b.pos;
        for (x, y) in b.offsets() {
            let p = pos + IVec2::new(*x, *y);
            if p.x < 0 || p.x >= self.size.x as i32 ||
                p.y < 0 || p.y >= self.size.y as i32 {
                // Outside of field
                return false;
            }

            let i = self.cell_index(p.x as usize, p.y as usize);
            if let Cell::Filled(_) = self.cells[i] {
                return false;
            }
        }
        true
    }

    pub fn move_active_block(&mut self, delta: IVec2) -> bool {
        match &self.active_block {
            None => { false }
            Some(b) => {
                let mut wb = b.clone();
                wb.pos += delta;
                let can_fit = self.can_fit_block(&wb);
                if can_fit {
                    self.active_block = Some(wb);
                }
                can_fit
            }
        }
    }

    pub fn commit_active_block(&mut self) {
        if let Some(b) = self.active_block {
            for (x, y) in b.offsets() {
                self.set_cell(
                    (*x + b.pos.x) as usize,
                    (*y + b.pos.y) as usize,
                    Cell::Filled(b.color))
            }
        }
    }

    pub fn drop_active_block(&mut self) -> TickResult {
        loop {
            match self.tick(0.0) {
                Updated => {}
                r => { return r; }
            }
        }
    }

    fn ghost_block(&self, block: Tetromino) -> Tetromino {
        let mut wb = block.clone();
        wb.color = WHITE;
        while self.can_fit_block(&wb) {
            wb.pos += IVec2::Y;
        }
        wb.pos += IVec2::NEG_Y;
        return wb;
    }

    pub fn render(&self) {
        let cell_size = CONFIG.cell_size;
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
            BLACK,
        );

        // Draw Active Block
        match &self.active_block {
            Some(b) => {
                let ghost = self.ghost_block(b.clone());
                render_tetromino(
                    ghost.pos.x as f32 * cell_size,
                    ghost.pos.y as f32 * cell_size,
                    cell_size,
                    &ghost,
                    true
                );
                render_tetromino(
                    b.pos.x as f32 * cell_size,
                    b.pos.y as f32 * cell_size,
                    cell_size,
                    b,
                    false
                );
            }
            _ => {}
        }

        // Draw existing field
        for y in 0..self.size.y as usize {
            for x in 0..self.size.x as usize {
                match self.cells[x + y * self.size.x as usize] {
                    Cell::Empty => {}
                    Cell::Filled(c) => {
                        render_cell(
                            x as f32 * cell_size,
                            y as f32 * cell_size,
                            cell_size,
                            c,
                            false,
                        )
                    }
                }
            }
        }

        // Draw line clearing
        if let FieldState::ClearingLines { from, to, progress } = self.state {
            draw_rectangle(
                0.0,
                from as f32 * cell_size,
                self.size.x as f32 * cell_size,
                (to - from + 1) as f32 * cell_size,
                WHITE,
            );

            let p = (progress / self.clear_time).clamp(0.0, 1.0);
            draw_rectangle(
                0.0,
                from as f32 * cell_size,
                self.size.x as f32 * cell_size * interpolate(p),
                (to - from + 1) as f32 * cell_size,
                BLACK,
            );
        }
    }
}

fn interpolate(x: f32) -> f32 {
    if x < 0.5 {
        2.0 * x * x
    } else {
        1.0 - (-2.0 * x + 2.0).powi(2) / 2.0
    }
}

fn render_tetromino(x: f32, y: f32, cell_size: f32, t: &Tetromino, outline: bool) {
    let offsets = t.offsets();
    for (x_offset, y_offset) in offsets {
        render_cell(
            x + *x_offset as f32 * cell_size,
            y + *y_offset as f32 * cell_size,
            cell_size,
            t.color,
            outline,
        )
    }
}

fn render_cell(x: f32, y: f32, cell_size: f32, color: Color, outline: bool) {
    if outline {
        draw_rectangle_lines(x, y, cell_size, cell_size, 2.0, color);
    } else {
        draw_rectangle(x, y, cell_size, cell_size, color)
    }
}

#[derive(Clone, Copy)]
pub enum Cell {
    Empty,
    Filled(Color),
}


#[derive(Copy, Clone)]
pub struct Tetromino {
    pub pos: IVec2,
    pub shape: Shape,
    pub rot: usize,
    pub color: Color,
}

impl Tetromino {
    pub fn rotation_count(&self) -> usize {
        return ROTATIONS[self.shape as usize].len();
    }

    pub fn offsets(&self) -> &[(i32, i32); 4] {
        return &ROTATIONS[self.shape as usize][self.rot];
    }
}

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum Shape {
    O,
    I,
    J,
    L,
    T,
    S,
    Z,
}

impl Shape {
    pub fn rand() -> Shape {
        let v = rand() % 7;
        match v {
            0 => Shape::O,
            1 => Shape::I,
            2 => Shape::J,
            3 => Shape::L,
            4 => Shape::T,
            5 => Shape::S,
            6 => Shape::Z,
            _ => { panic!("Shouldn't get here") }
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Shape::O => YELLOW,
            Shape::I => SKYBLUE,
            Shape::J => DARKBLUE,
            Shape::L => ORANGE,
            Shape::T => PURPLE,
            Shape::S => GREEN,
            Shape::Z => RED,
        }
    }
}

lazy_static! {
    static ref ROTATIONS : Vec<Vec<[(i32, i32); 4]>> = vec!(
        // O
        vec!([(0,0), (1, 0), (0, 1), (1, 1)]),
        // I
        vec!(
            [(0,-1), (0, 0), (0, 1), (0, 2)],
            [(-1,0), (0, 0), (1, 0), (2, 0)]
        ),
        // J
        vec!(
            [(0,-1), (0, 0), (0, 1), (-1, 1)],
            [(-1,0), (0, 0), (1, 0), (1, 1)],
            [(0,1), (0, 0), (0, -1), (1, -1)],
            [(1,0), (0, 0), (-1, 0), (-1, -1)],
        ),
        // L
        vec!(
            [(0,-1), (0, 0), (0, 1), (1, 1)],
            [(-1,0), (0, 0), (1, 0), (1, -1)],
            [(0,1), (0, 0), (0, -1), (-1, -1)],
            [(1,0), (0, 0), (-1, 0), (-1, 1)],
        ),
        // T
        vec!(
            [(-1,0), (0, 0), (0, -1), (0, 1)],
            [(-1,0), (0, 0), (0, 1), (1, 0)],
            [(0,1), (0, 0), (0, -1), (1, 0)],
            [(-1,0), (0, 0), (0, -1), (1, 0)],
        ),
        // S
        vec!(
            [(0,-1), (0, 0), (1, 0), (1, 1)],
            [(1,0), (0, 0), (0, 1), (-1, 1)],
        ),
        // Z
        vec!(
            [(0,-1), (0, 0), (-1, 0), (-1, 1)],
            [(-1,0), (0, 0), (0, 1), (1, 1)],
        ),
    );
}
