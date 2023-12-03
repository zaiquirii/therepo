use cgmath::Zero;

use super::math::{Aabb2, Vec2};

pub type EntityId = u32;

#[derive(Clone, Copy)]
pub struct PointMass {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub mass: f32,
}

impl PointMass {
    pub fn new(position: Vec2, mass: f32) -> Self {
        Self {
            position,
            velocity: Vec2::zero(),
            acceleration: Vec2::zero(),
            mass,
        }
    }

    pub fn pos(&self) -> Vec2 {
        self.position
    }

    pub fn velocity(&self) -> Vec2 {
        self.velocity
    }

    pub fn apply_force(&mut self, impulse: Vec2) {
        self.acceleration += impulse / self.mass;
    }

    pub fn clear_forces(&mut self) {
        self.acceleration = Vec2::zero();
    }

    pub fn update_position(&mut self, delta: f32) {
        self.velocity += self.acceleration * delta;
        self.position += self.velocity * delta;
        // self.curr_pos += self.velocity * delta + 0.5 * self.acceleration * delta * delta;
        // self.velocity += self.acceleration * delta;

        // let velocity = self.curr_pos - self.prev_pos;
        // self.prev_pos = self.curr_pos;
        // self.curr_pos = self.curr_pos + velocity + self.acceleration * delta * delta;
        // self.velocity = velocity / delta;
        // TODO: move acceleration reset here
    }
}

pub struct Spring {
    pub a_index: usize,
    pub b_index: usize,
    pub stiffness: f32,
    pub length: f32,
    pub damping: f32,
}

impl Spring {}

pub struct SoftBody {
    pub id: EntityId,
    pub points: Vec<PointMass>,
    pub springs: Vec<Spring>,
    pub aabb: Aabb2,
    pub is_dynamic: bool,
}

impl SoftBody {
    pub fn new(points: Vec<PointMass>, springs: Vec<Spring>, is_dynamic: bool) -> Self {
        Self {
            id: 0,
            points,
            springs,
            aabb: Aabb2::zero(),
            is_dynamic,
        }
    }
}

pub struct SoftBodyTemplate {
    pub points: Vec<PointMass>,
    pub springs: Vec<(usize, usize)>,
    pub is_dynamic: bool,
}
