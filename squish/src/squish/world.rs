use super::{
    entities::{EntityId, PointMass, SoftBody, SoftBodyTemplate},
    math::{Aabb2, Vec2},
};

pub struct World {
    softbodies: Vec<SoftBody>,
}

impl World {
    pub fn new() -> World {
        World {
            softbodies: Vec::new(),
        }
    }

    pub fn create_softbody(&mut self, template: &SoftBodyTemplate, offset: Vec2) -> EntityId {
        let new_id = self.softbodies.len() as EntityId;
        let softbody = SoftBody {
            id: new_id,
            points: template
                .points
                .iter()
                .map(|p| PointMass::new(p.curr_pos + offset, p.mass))
                .collect(),
            aabb: Aabb2::zero(),
            is_dynamic: template.is_dynamic,
        };
        self.softbodies.push(softbody);
        new_id
    }

    pub fn softbodies(&self) -> &Vec<SoftBody> {
        &self.softbodies
    }

    pub fn pointmasses(&self) -> Vec<PointMass> {
        let mut points = Vec::new();
        for body in &self.softbodies {
            points.extend_from_slice(body.points.as_slice());
        }
        points
    }

    pub fn update(&mut self, delta: f32) {
        self.clear_forces();
        self.apply_gravity();
        self.solve_collisions();
        self.update_positions(delta);
    }

    fn clear_forces(&mut self) {
        for softbody in &mut self.softbodies {
            for point in &mut softbody.points {
                point.clear_forces();
            }
        }
    }

    fn apply_gravity(&mut self) {
        let gravity = Vec2::new(0.0, -9.0);
        for softbody in &mut self.softbodies {
            for point in &mut softbody.points {
                point.acceleration += gravity;
            }
        }
    }

    fn update_positions(&mut self, delta: f32) {
        for softbody in &mut self.softbodies {
            if softbody.is_dynamic {
                for point in &mut softbody.points {
                    point.update_position(delta);
                }
            }
        }
    }

    fn solve_collisions(&mut self) {
        let softbodies = &self.softbodies;
        for index in 0..softbodies.len() {}
    }
}
