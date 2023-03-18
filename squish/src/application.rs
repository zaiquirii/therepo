use std::time::{Duration, Instant};

use cgmath::Vector2;
use log::info;

use crate::{
    rendering::{
        camera::Camera2d,
        renderer::{Canvas, RectInstance, Renderer, Vertex},
    },
    squish::world::World,
};
pub struct Application {
    pub world: World,
    pub camera: Camera2d,
    pub renderer: Renderer,

    accumulated_time: Duration,
    last_update: Instant,
}

impl Application {
    pub fn new(world: World, renderer: Renderer, camera: Camera2d) -> Self {
        Self {
            world,
            renderer,
            camera,
            accumulated_time: Duration::new(0, 0),
            last_update: Instant::now(),
        }
    }
    pub fn handleInputEvent(&mut self) -> bool {
        false
    }

    pub fn update(&mut self) {
        let now = std::time::Instant::now();
        let delta = now.duration_since(self.last_update);
        self.last_update = now;
        self.accumulated_time += delta;

        let target_delta = 1.0 / 60.0;
        while (self.accumulated_time > Duration::from_secs_f32(target_delta)) {
            self.step(target_delta);
            self.accumulated_time -= Duration::from_secs_f32(target_delta);
        }
    }

    fn step(&mut self, delta: f32) {
        self.world.update(delta);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.renderer.set_camera(&self.camera);
        let mut canvas = Canvas::new();

        for softbody in self.world.softbodies() {
            // Only draw lines where there are points to connect
            if !softbody.points.len() > 1 {
                let mut prev_pos = softbody.points[softbody.points.len() - 1].curr_pos;
                for pointmass in &softbody.points {
                    let pos = pointmass.curr_pos;
                    canvas.draw_line(prev_pos, pos, 0.3, wgpu::Color::WHITE);

                    prev_pos = pos;
                }
            }

            for pointmass in &softbody.points {
                let pos = pointmass.curr_pos;
                canvas.draw_rectangle(
                    pos.x - 0.5,
                    pos.y + 0.5,
                    pos.x + 0.5,
                    pos.y - 0.5,
                    wgpu::Color::RED,
                );
            }

            for spring in &softbody.springs {
                canvas.draw_line(
                    softbody.points[spring.a_index].pos(),
                    softbody.points[spring.b_index].pos(),
                    0.3,
                    wgpu::Color::GREEN,
                );
            }

            // canvas.draw_rectangle(
            //     softbody.aabb.left(),
            //     softbody.aabb.top(),
            //     softbody.aabb.right(),
            //     softbody.aabb.bottom(),
            //     wgpu::Color::GREEN,
            // );
        }

        match self.renderer.render(&canvas) {
            Err(wgpu::SurfaceError::Lost) => {
                self.renderer.resize(self.renderer.window_size);
                Ok(())
            }
            result => result,
        }
    }
}
