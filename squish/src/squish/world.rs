use cgmath::{assert_abs_diff_eq, vec2, Bounded};
use cgmath::{InnerSpace, Zero};

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
            .map(|p| PointMass::new(p.position + offset, p.mass))
            .collect();
        let springs = template
            .springs
            .iter()
            .map(|(a_index, b_index)| Spring {
                a_index: *a_index,
                b_index: *b_index,
                stiffness: 100.0,
                damping: 3.0,
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
            softbody.aabb.reset(softbody.points[0].position);
            for index in 1..softbody.points.len() {
                softbody.aabb.expand(softbody.points[index].position);
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
        for point in &mut left.points {
            let probe = LineSegment::new(
                point.pos(),
                Vec2::new(right.aabb.left() - 1.0, point.pos().y),
            );
            let mut closest_point = vec2(4000.0, 4000.0);
            let mut closest_index = 0;
            let mut min_distance_squared = f32::MAX;
            let mut intersections = 0;

            // TODO: remove this allocation
            let segments = segments_from_points(&right.points);
            for (segment_index, segment) in segments.iter().enumerate() {
                use cgmath::InnerSpace;
                let local_closest_point = segment.closest_point(point.pos());
                let distance_squared = (local_closest_point - point.pos()).magnitude2();
                if min_distance_squared > distance_squared {
                    min_distance_squared = distance_squared;
                    closest_point = local_closest_point;
                    closest_index = segment_index;
                }

                if probe.intersects(segment) {
                    intersections += 1;
                }
            }

            // Handle collision
            if intersections % 2 == 1 {
                let raw_vector = closest_point - point.position;
                // If the raw vector length is 0 the point hasn't moved so we'll skip and get
                // it on the next step.
                if raw_vector.magnitude2() == 0.0 {
                    continue;
                }

                let right_len = right.points.len();
                let [a, b] = unsafe {
                    right
                        .points
                        .get_many_unchecked_mut([closest_index, (closest_index + 1) % right_len])
                };

                let og_momentum = a.velocity + b.velocity + point.velocity;

                if right.is_dynamic {
                    let line_velocity = (a.velocity + b.velocity) / 2.0;
                    let tangent = raw_vector.normalize();
                    // let tangent = vec2(raw_vector.y, -raw_vector.x).normalize();

                    let p = 2.0 * (point.velocity.dot(tangent) - line_velocity.dot(tangent)) / 3.0;
                    let t = (closest_point - a.position).magnitude()
                        / (b.position - a.position).magnitude();
                    // Magic math for the win, its in my notebook somewhere
                    let z = 1.0 - (2.0 * t - 1.0).powi(2);

                    // Update Velocities
                    point.velocity -= p * tangent;

                    let output_line_velocity = p * 4.0 * tangent;
                    a.velocity += t * output_line_velocity;
                    b.velocity += t * output_line_velocity;

                    // Update Positions
                    point.position += raw_vector * (z / 3.0 + 0.5);
                    a.position -= raw_vector * (0.5 * t);
                    b.position -= raw_vector * (0.5 * (1.0 - t));
                } else {
                    // TODO: Change this to conserve momentum
                    let collision_norm = raw_vector.normalize();
                    let p = 2.0 * (point.velocity.dot(collision_norm));
                    point.velocity -= p * collision_norm;
                    point.position = closest_point;
                }

                let end_momentum = a.velocity + b.velocity + point.velocity;
                if ((end_momentum.magnitude() / og_momentum.magnitude()) - 1.0).abs() > 0.1 {
                    println!(
                        "{} {} {} {}",
                        og_momentum.magnitude(),
                        end_momentum.magnitude(),
                        end_momentum.magnitude() - og_momentum.magnitude(),
                        end_momentum.magnitude() / og_momentum.magnitude()
                    );
                }

                // println!(
                //     "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
                //     vector,
                //     tangent,
                //     tangent_speed,
                //     tangent_velocity_component,
                //     perpendicular_velocity_component,
                //     output_line_velocity,
                //     closest_point
                // )
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
