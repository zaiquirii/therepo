use cgmath::{InnerSpace, Zero};

pub type Vec2 = cgmath::Vector2<f32>;

pub struct Aabb2 {
    pub min: Vec2,
    pub max: Vec2,
}

impl Aabb2 {
    pub fn zero() -> Self {
        Self {
            min: Vec2::zero(),
            max: Vec2::zero(),
        }
    }

    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    pub fn from_point(point: Vec2) -> Self {
        Self {
            min: point,
            max: point,
        }
    }

    pub fn right(&self) -> f32 {
        self.max.x
    }
    pub fn left(&self) -> f32 {
        self.min.x
    }
    pub fn top(&self) -> f32 {
        self.max.y
    }
    pub fn bottom(&self) -> f32 {
        self.min.y
    }

    pub fn reset(&mut self, point: Vec2) {
        self.min = point;
        self.max = point;
    }

    pub fn expand(&mut self, point: Vec2) {
        if point.x < self.min.x {
            self.min.x = point.x;
        } else if point.x > self.max.x {
            self.max.x = point.x;
        }

        if point.y < self.min.y {
            self.min.y = point.y;
        } else if point.y > self.max.y {
            self.max.y = point.y;
        }
    }
}

pub struct LineSegment {
    pub start: Vec2,
    pub end: Vec2,
}

impl LineSegment {
    pub fn new(start: Vec2, end: Vec2) -> Self {
        Self { start, end }
    }

    pub fn midpoint(&self) -> Vec2 {
        self.start + (self.end - self.start) * 0.5
    }

    pub fn intersects(&self, other: &LineSegment) -> bool {
        let o1 = orientation(self.start, self.end, other.start);
        let o2 = orientation(self.start, self.end, other.end);

        let o3 = orientation(other.start, other.end, self.start);
        let o4 = orientation(other.start, other.end, self.end);

        return (o1 != o2) && (o3 != o4);
    }

    pub fn closest_point(&self, other: Vec2) -> Vec2 {
        let v = self.end - self.start;
        let u = self.start - other;
        let t = -v.dot(u) / v.dot(v);

        if t < 0.0 {
            self.start
        } else if t > 1.0 {
            self.end
        } else {
            (1.0 - t) * self.start + t * self.end
        }
    }
}

fn orientation(p: Vec2, q: Vec2, r: Vec2) -> u8 {
    let result = ((q.y - p.y) * (r.x - q.x)) - ((q.x - p.x) * (r.y - q.y));
    if result > 0.0 {
        return 1;
    }
    if result < 0.0 {
        return 2;
    }
    return 0;
}

#[cfg(test)]
mod line_segment_tests {
    use cgmath::vec2;

    use super::{LineSegment, Vec2};

    fn intersect_test(points: [f32; 8], should_intersect: bool) {
        let segment_one = LineSegment::new(
            Vec2::new(points[0], points[1]),
            Vec2::new(points[2], points[3]),
        );
        let segment_two = LineSegment::new(
            Vec2::new(points[4], points[5]),
            Vec2::new(points[6], points[7]),
        );

        assert_eq!(segment_one.intersects(&segment_two), should_intersect);
    }

    #[test]
    fn intersects_works() {
        intersect_test([0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0], true);
        intersect_test([0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0], false);
    }

    #[test]
    fn closest_point_works() {
        let segment = LineSegment::new(vec2(0.0, 0.0), vec2(20.0, 0.0));

        assert_eq!(segment.closest_point(vec2(10.0, 10.0)), vec2(10.0, 0.0));
        assert_eq!(segment.closest_point(vec2(-10.0, 10.0)), vec2(0.0, 0.0));
        assert_eq!(segment.closest_point(vec2(40.0, 10.0)), vec2(20.0, 0.0));
    }
}
