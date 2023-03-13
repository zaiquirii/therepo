use cgmath::Zero;

pub type Vec2 = cgmath::Vector2<f32>;

pub struct Aabb2 {
    min: Vec2,
    max: Vec2,
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
}
