use cgmath::{Matrix4, Vector2};

pub struct Camera2d {
    pub location: Vector2<f32>,
    pub scale: f32,
}

impl Camera2d {
    pub fn new(location: Vector2<f32>, scale: f32) -> Self {
        Self { location, scale }
    }

    pub fn projection(&self, aspect_ratio: f32) -> Matrix4<f32> {
        // scale is half height of screen in world units
        // a scale of 1 should result in a camera view that is two world units tall

        let half_height = self.scale;
        let half_width = half_height * aspect_ratio;

        let proj = cgmath::ortho(
            self.location.x - half_width,
            self.location.x + half_width,
            self.location.y + self.scale,
            self.location.y - self.scale,
            10.0,
            -10.0,
        );
        MIRROR_Y_MATRIX * proj
    }
}

#[rustfmt::skip]
pub const MIRROR_Y_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, -1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 1.0,
);

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_prof: [[f32; 4]; 4],
}
