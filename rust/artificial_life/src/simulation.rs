use macroquad::hash;
use macroquad::math::{Rect, Vec2};
use macroquad::prelude::*;
use macroquad::rand::gen_range;
use macroquad::shapes::draw_circle;
use macroquad::ui::root_ui;
use macroquad::ui::widgets::{Group, TreeNode, Window};
use crate::grid::Grid;
use crate::time::MovingAverage;

#[derive(Clone)]
pub struct SpeciesConfig {
    range: f32,
    attraction: Vec<f32>,
}

#[derive(Debug)]
pub struct Atom {
    species: u8,
    pos: Vec2,
    vel: Vec2,
}

pub struct SimulationConfig {
    fps: f32,
    atoms_per_species: usize,
    viscosity: f32,
    gravity: f32,
    force_const: f32,
    num_species: u8,
    species_config: Vec<SpeciesConfig>,
}

pub struct Simulation {
    atoms: Vec<Atom>,
    bounds: Rect,
    config: SimulationConfig,
    grid: Grid<usize>,
    grids: Vec<Grid<usize>>,
    avg_fps: MovingAverage,
}

impl Simulation {
    pub fn new(bounds: Rect) -> Self {
        let config = SimulationConfig {
            atoms_per_species: 400,
            num_species: 2,
            fps: 60.0,
            viscosity: 0.5,
            gravity: 0.0,
            force_const: 1.0,
            species_config: vec![
                SpeciesConfig {
                    range: 80.0,
                    attraction: vec![-1.0; 8],
                }; 8],
        };

        let mut s = Self {
            bounds,
            config,
            atoms: Vec::new(),
            grid: Grid::new(0, bounds, UVec2::new(10, 10)),
            grids: Vec::with_capacity(8),
            avg_fps: MovingAverage::new(100),
        };
        s.reconcile_config(true);
        s
    }

    pub fn fps(&self) -> f32 {
        self.config.fps
    }

    fn reconcile_config(&mut self, force: bool) {
        let atom_count = self.config.atoms_per_species * self.config.num_species as usize;
        if force || self.atoms.len() != atom_count {
            let num_species = self.config.num_species as usize;
            self.atoms = generate_atoms(self.bounds,
                                        self.config.num_species,
                                        self.config.atoms_per_species);
            let configs = &mut self.config.species_config;
            if configs.len() > num_species {
                configs.truncate(num_species);
            }
            for i in 0..num_species {
                if configs.len() == i {
                    configs.push(SpeciesConfig {
                        range: 150.0,
                        attraction: vec![0.0; num_species],
                    })
                } else {
                    let attraction = &mut configs[i].attraction;
                    attraction.truncate(num_species);

                    if attraction.len() < num_species {
                        for _ in attraction.len()..num_species {
                            attraction.push(0.0);
                        }
                    }
                }
            }
            let mut bounds = self.bounds.clone().offset(Vec2::new(-2.0, -2.0));
            bounds.w += 4.0;
            bounds.h += 4.0;
            self.grid = Grid::new(atom_count, bounds, UVec2::new(10, 10));
            for i in 0..self.config.num_species as usize {
                let g = Grid::new(self.config.atoms_per_species, bounds, UVec2::new(10, 10));
                if self.grids.len() <= i {
                    self.grids.push(g);
                } else {
                    self.grids[i] = g;
                }
            }
        }
    }

    pub fn tick(&mut self) {
        let num_species = self.config.num_species as usize;
        self.grid.reset();
        for a_i in 0..self.atoms.len() {
            self.grid.insert(self.atoms[a_i].pos, a_i);
        }
        self.grid.finalize();

        let mut remaining = self.atoms.as_mut_slice();
        let mut species: [&mut [Atom]; 8] = Default::default();
        for s in 0..(num_species - 1) as usize {
            let (head, tail) = remaining.split_at_mut(self.config.atoms_per_species);
            species[s] = head;
            remaining = tail;
        }
        species[num_species - 1] = remaining;

        for s in 0..num_species {
            let grid = &mut self.grids[s];
            grid.reset();
            for (index, atom) in species[s].iter().enumerate() {
                grid.insert(atom.pos, index);
            }
            grid.finalize();
        }

        for a_species_i in 0..num_species {
            let config = &self.config.species_config[a_species_i];
            let forces = &config.attraction;
            let range = config.range;

            for a_i in 0..self.config.atoms_per_species {
                let a_pos = species[a_species_i][a_i].pos;
                let mut acc_force = Vec2::ZERO;

                for b_species_i in 0..num_species {
                    let b_force = forces[b_species_i];
                    if b_force == 0.0 {
                        continue;
                    }

                    let b_species = &species[b_species_i];
                    for cell in self.grids[b_species_i].scan(a_pos, range) {
                        for (_, b_i) in cell {
                            let delta = a_pos - b_species[*b_i].pos;
                            let distance = delta.length();
                            if distance > 0.0 && distance < config.range {
                                let force = b_force / distance;
                                acc_force += force * delta;
                            }
                        }
                    }
                }

                let a = &mut species[a_species_i][a_i];
                a.vel = (a.vel + acc_force * self.config.force_const) * self.config.viscosity;
                a.vel.y += self.config.gravity;
            }
        }

        for a in &mut self.atoms {
            a.pos += a.vel;
            if a.pos.x <= self.bounds.left() {
                a.vel.x *= -1.0;
                a.pos.x = self.bounds.left();
            } else if a.pos.x >= self.bounds.right() {
                a.vel.x *= -1.0;
                a.pos.x = self.bounds.right();
            } else if a.pos.y <= self.bounds.top() {
                a.vel.y *= -1.0;
                a.pos.y = self.bounds.top();
            } else if a.pos.y >= self.bounds.bottom() {
                a.vel.y *= -1.0;
                a.pos.y = self.bounds.bottom();
            }
        }
    }

    pub fn render(&self) {
        let colors = [YELLOW, RED, GREEN, BLUE, PURPLE, ORANGE, MAGENTA, VIOLET];
        for a in &self.atoms {
            draw_circle(a.pos.x, a.pos.y, 3.0, colors[a.species as usize])
        }
    }

    pub fn render_grid(&self, select_point: Option<Vec2>) {
        let cell = self.grid.cell_size;
        for y in 0..self.grid.dimens.y {
            for x in 0..self.grid.dimens.x {
                let x_f = x as f32 * cell.x + self.grid.origin.x;
                let y_f = y as f32 * cell.y + self.grid.origin.y;
                draw_rectangle_lines(x_f, y_f, cell.x, cell.y, 2.0, WHITE);
            }
        }

        if let Some(p) = select_point {
            let x_f = (p.x / cell.x).floor() * cell.x + self.grid.origin.x;
            let y_f = (p.y / cell.y).floor() * cell.y + self.grid.origin.y;
            draw_rectangle_lines(x_f, y_f, cell.x, cell.y, 4.0, GREEN);

            for cell in self.grid.scan(p, 80.0) {
                for (_, index) in cell {
                    let a = &self.atoms[*index];
                    draw_circle(a.pos.x, a.pos.y, 3.0, GREEN)
                }
            }
        }
    }

    pub fn render_ui(&mut self) {
        root_ui().label(None, "TEST");

        let names = ["Yellow", "Red", "Green", "Blue", "Purple", "Orange", "Magenta", "Violet"];

        // Group::new(hash!(), Vec2::new(screen_width() - 400.0, 0.0))
        Window::new(hash!(), Vec2::new(screen_width() - 400.0, 0.0), Vec2::new(400.0, screen_height()))
            .movable(false)
            .label("Config")
            .ui(&mut root_ui(), |ui| {
                self.avg_fps.update(get_fps());
                ui.label(None, &*format!("FPS: {}", self.avg_fps.avg()));
                if ui.button(None, "Reset Particles") {
                    self.reconcile_config(true);
                }
                let mut num_species = Some(self.config.num_species as usize - 1);
                ui.combo_box(hash!(), "Species Count", &["One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight"], &mut num_species);
                if let Some(s) = num_species {
                    let num = s as u8 + 1;
                    if self.config.num_species != num {
                        self.config.num_species = num;
                        self.reconcile_config(false);
                    }
                }
                let mut value = self.config.atoms_per_species as f32;
                ui.slider(hash!(), "Framerate", 0.0..120.0, &mut self.config.fps);
                ui.slider(hash!(), "Atoms per species", 1.0..2000.0, &mut value);
                if value as usize != self.config.atoms_per_species {
                    self.config.atoms_per_species = value as usize;
                    self.reconcile_config(false);
                }
                ui.slider(hash!(), "Viscosity", 0.0..1.0, &mut self.config.viscosity);
                ui.slider(hash!(), "Gravity", 0.0..4.0, &mut self.config.gravity);
                ui.slider(hash!(), "Force Const", 0.0..10.0, &mut self.config.force_const);
                ui.separator();

                TreeNode::new(hash!(), "Species Config")
                    .init_unfolded()
                    .ui(ui, |ui| {
                        for species in 0..self.config.num_species as usize {
                            TreeNode::new(hash!("species", species), names[species])
                                .init_unfolded()
                                .ui(ui, |ui| {
                                    ui.slider(hash!("range", species), "Range", 0.0..1000.0, &mut self.config.species_config[species].range);
                                    for other in 0..self.config.num_species as usize {
                                        ui.slider(hash!("attraction", species + 500, other),
                                                  &*format!("{} -> {}", names[species].chars().next().unwrap(), names[other].chars().next().unwrap()),
                                                  -3.0..3.0,
                                                  &mut self.config.species_config[species].attraction[other])
                                    }
                                });
                        }
                    });
            });
    }
}

fn generate_atoms(bounds: Rect, num_species: u8, atoms_per_species: usize) -> Vec<Atom> {
    let mut atoms = Vec::with_capacity(num_species as usize * atoms_per_species);
    for s in 0..num_species {
        for _ in 0..atoms_per_species {
            atoms.push(Atom {
                species: s,
                pos: Vec2::new(
                    gen_range(bounds.left(), bounds.right()),
                    gen_range(bounds.top(), bounds.bottom()),
                ),
                vel: Vec2::ZERO,
            })
        }
    }
    atoms
}