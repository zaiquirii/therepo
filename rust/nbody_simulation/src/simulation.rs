use std::f32::consts::PI;
use macroquad::math::{Rect, Vec2};
use macroquad::rand::{gen_range};
use crate::quadtree::Quadtree;

const GRAVITY_CONST: f32 = 25.0;
const MIN_DISTANCE: f32 = 0.0001;

#[derive(Copy, Clone)]
pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub force: Vec2,
    pub size: f32,
    pub mass: f32,
}

pub struct Impulser {
    pos: Vec2,
    force: f32,
    range: f32,
}

impl Impulser {
    pub fn new(pos: Vec2, force: f32, range: f32) -> Self {
        Self {
            pos,
            force,
            range,
        }
    }
}

pub struct Simulation {
    bodies: Vec<Body>,
    bounds: Rect,
    impulser: Option<Impulser>,
    qt: Option<Quadtree<usize>>,
}

impl Simulation {
    pub fn new(body_count: usize, bounds: Rect) -> Self {
        let mut bodies = Vec::with_capacity(body_count);
        let radius = bounds.w.min(bounds.y) / 2.0;
        for i in 0..body_count {
            let pos = Vec2::from_angle(gen_range(0.0, 2.0 * PI))
                .rotate(Vec2::new(
                    // 2.0 / radius.powi(2) * gen_range(0.0, radius),
                    radius,
                    0.0,
                ));
            // Vec2::new(i as f32 * 15.0, 0.0).rotat
            bodies.push(Body {
                // pos: Vec2::new(i as f32 * 20.0, i as f32 * 15.0),
                // vel: Vec2::new(i as f32 * 5.0, 0.0),
                pos,
                vel: Vec2::ZERO,
                force: Vec2::ZERO,
                size: 3.0,
                mass: 1.0,
            })
        }

        Self {
            bodies,
            bounds,
            qt: None,
            impulser: None,
        }
    }

    pub fn add_impulser(&mut self, impulser: Impulser) {
        self.impulser = Some(impulser);
    }

    pub fn tick(&mut self, dt: f32) {
        let mut qt = Quadtree::new(self.bounds, 20);

        for a in 0..self.bodies.len() {
            let mut a_bod = self.bodies[a];
            qt.insert(a_bod.pos, a);
            let mut acc_force = Vec2::ZERO;
            for (b, b_bod) in self.bodies.iter().enumerate() {
                if a == b {
                    continue;
                }

                let pos_delta = b_bod.pos - a_bod.pos;
                let length_squared = pos_delta.length();
                if length_squared == 0.0 {
                    continue;
                }
                let force_mag = GRAVITY_CONST / length_squared.max(MIN_DISTANCE);
                let new_force = a_bod.mass * b_bod.mass * pos_delta.normalize() * force_mag;
                acc_force += new_force;
            }
            self.bodies[a].force = acc_force;
        }

        if let Some(impulser) = &self.impulser {
            self.bodies.iter_mut()
                .for_each(|b| {
                    let pos_delta = impulser.pos - b.pos;
                    let length_squared = pos_delta.length();
                    if length_squared == 0.0 {
                        return;
                    }
                    let force_mag = GRAVITY_CONST / length_squared.max(MIN_DISTANCE);
                    let new_force = b.mass * impulser.force * pos_delta.normalize() * force_mag;
                    b.force += new_force;
                })
        }

        self.apply_forces(dt);

        self.qt = Some(qt);
        self.impulser = None
    }

    fn apply_forces(&mut self, dt: f32) {
        for bod in &mut self.bodies {
            bod.vel = (bod.vel +  bod.force * dt) * (1.0 - (0.999 * dt));
            bod.pos += bod.vel * dt;

            if bod.pos.x < self.bounds.left() {
                bod.pos.x = self.bounds.right();
            } else if bod.pos.x > self.bounds.right() {
                bod.pos.x = self.bounds.left()
            }
            if bod.pos.y < self.bounds.top() {
                bod.pos.y = self.bounds.bottom();
            } else if bod.pos.y > self.bounds.bottom() {
                bod.pos.y = self.bounds.top()
            }
        }
    }

    pub fn bodies(&self) -> &[Body] {
        self.bodies.as_slice()
    }

    pub fn qt(&self) -> Option<&Quadtree<usize>> {
        self.qt.as_ref()
    }
}