use cgmath::InnerSpace;
use cgmath::{vec2, Bounded};

use super::{
    entities::{EntityId, PointMass, SoftBody, SoftBodyTemplate, Spring},
    math::{Aabb2, LineSegment, Vec2},
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
        let points: Vec<PointMass> = template
            .points
            .iter()
            .map(|p| PointMass::new(p.curr_pos + offset, p.mass))
            .collect();
        let springs = template
            .springs
            .iter()
            .map(|(a_index, b_index)| Spring {
                a_index: *a_index,
                b_index: *b_index,
                stiffness: 20.0,
                damping: 8.0,
                length: (points[*b_index].pos() - points[*a_index].pos()).magnitude(),
            })
            .collect();

        let softbody = SoftBody {
            id: new_id,
            points,
            springs,
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
        self.apply_gravity();
        self.update_aabbs();
        self.solve_collisions();
        self.solve_springs();
        self.update_positions(delta);
        self.clear_forces();
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

    fn update_aabbs(&mut self) {
        for softbody in &mut self.softbodies {
            softbody.aabb.reset(softbody.points[0].curr_pos);
            for index in 1..softbody.points.len() {
                softbody.aabb.expand(softbody.points[index].curr_pos);
            }
        }
    }

    fn solve_collisions(&mut self) {
        let softbodies = &mut self.softbodies;
        for lhs in 0..softbodies.len() {
            for rhs in lhs + 1..softbodies.len() {
                let [left, right] = unsafe { softbodies.get_many_unchecked_mut([lhs, rhs]) };
                Self::resolve_softbody_collisions(left, right);
            }
        }
    }

    fn resolve_softbody_collisions(left: &mut SoftBody, right: &mut SoftBody) {
        if left.is_dynamic {
            Self::resolve_softbody_points(left, right);
        }
        if right.is_dynamic {
            Self::resolve_softbody_points(right, left);
        }
    }

    // Move points from left if inside of right
    fn resolve_softbody_points(left: &mut SoftBody, right: &mut SoftBody) {
        let segments = segments_from_points(&right.points);
        for point in &mut left.points {
            let probe = LineSegment::new(
                point.pos(),
                Vec2::new(right.aabb.left() - 1.0, point.pos().y),
            );
            let mut closest_point = vec2(4000.0, 4000.0);
            let mut min_distance_squared = f32::MAX;
            let mut intersections = 0;

            for segment in &segments {
                use cgmath::InnerSpace;
                let local_closest_point = segment.closest_point(point.pos());
                let distance_squared = (local_closest_point - point.pos()).magnitude2();
                if min_distance_squared > distance_squared {
                    min_distance_squared = distance_squared;
                    closest_point = local_closest_point;
                }

                if probe.intersects(segment) {
                    intersections += 1;
                }
            }

            if intersections % 2 == 1 {
                println!(
                    "moving point from {:?} to {:?}",
                    point.curr_pos, closest_point
                );
                point.curr_pos = closest_point;
            }
        }
    }

    fn solve_springs(&mut self) {
        for softbody in &mut self.softbodies {
            for spring in &softbody.springs {
                let a = softbody.points[spring.a_index];
                let b = softbody.points[spring.b_index];

                let force_norm = (b.pos() - a.pos()).normalize();

                let damping_force = force_norm.dot(b.velocity() - a.velocity()) * spring.damping;
                let spring_force =
                    ((b.pos() - a.pos()).magnitude() - spring.length) * spring.stiffness;

                let total_force = damping_force + spring_force;

                softbody.points[spring.a_index].apply_force(force_norm * total_force);
                softbody.points[spring.b_index].apply_force(force_norm * -total_force);
            }
        }
    }
}

fn segments_from_points(points: &[PointMass]) -> Vec<LineSegment> {
    let mut segments = Vec::new();
    for index in 0..points.len() {
        segments.push(LineSegment::new(
            points[index].pos(),
            points[(index + 1) % points.len()].pos(),
        ));
    }
    segments
}
