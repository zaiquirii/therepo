use std::time::{Duration, Instant};

use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};

use crate::{
    rendering::{
        camera::Camera2d,
        renderer::{Canvas, Renderer},
    },
    squish::{
        entities::{PointMass, SoftBodyTemplate},
        math::LineSegment,
        world::World,
    },
};
pub struct Application {
    pub world: World,
    pub camera: Camera2d,
    pub renderer: Renderer,
    count: i32,

    accumulated_time: Duration,
    last_update: Instant,
}

impl Application {
    pub fn new(mut world: World, renderer: Renderer, camera: Camera2d) -> Self {
        let template = SoftBodyTemplate {
            points: vec![
                PointMass::new((0.0, 0.0).into(), 1.0),
                PointMass::new((0.0, 10.0).into(), 1.0),
                PointMass::new((10.0, 10.0).into(), 1.0),
                PointMass::new((10.0, 0.0).into(), 1.0),
            ],
            springs: vec![(0, 1), (1, 2), (2, 3), (3, 0), (0, 2), (1, 3)],
            is_dynamic: true,
        };
        world.create_softbody(&template, (0.0, 0.0).into());
        // world.create_softbody(&template, (5.0, 20.0).into());

        world.create_softbody(
            &SoftBodyTemplate {
                points: vec![
                    PointMass::new((0.0, 0.0).into(), 1.0),
                    // PointMass::new((0.0, 40.0).into(), 1.0),
                    PointMass::new((0.0, 20.0).into(), 1.0),
                    // PointMass::new((50.0, 10.0).into(), 1.0),
                    PointMass::new((100.0, 20.0).into(), 1.0),
                    PointMass::new((100.0, 0.0).into(), 1.0),
                ],
                springs: Vec::new(),
                is_dynamic: false,
            },
            (-40.0, -40.0).into(),
        );
        Self {
            world,
            renderer,
            camera,
            count: 0,
            accumulated_time: Duration::new(0, 0),
            last_update: Instant::now(),
        }
    }
    pub fn handle_input_event(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Space),
                        ..
                    },
                ..
            } => {
                self.count += 1;
                let template = SoftBodyTemplate {
                    points: vec![
                        PointMass::new((0.0, 0.0).into(), 1.0),
                        PointMass::new((0.0, 10.0).into(), 1.0),
                        PointMass::new((10.0, 10.0).into(), 1.0),
                        PointMass::new((10.0, 0.0).into(), 1.0),
                    ],
                    springs: vec![(0, 1), (1, 2), (2, 3), (3, 0), (0, 2), (1, 3)],
                    is_dynamic: true,
                };
                self.world.create_softbody(
                    &template,
                    (-10.0 + 5.0 * (self.count % 5) as f32, 40.0).into(),
                );
                true
            }
            _ => false,
        }
    }

    pub fn update(&mut self) {
        let now = std::time::Instant::now();
        let delta = now.duration_since(self.last_update);
        self.last_update = now;
        self.accumulated_time += delta;

        let target_delta = 1.0 / 60.0;
        while self.accumulated_time > Duration::from_secs_f32(target_delta) {
            self.step(target_delta);
            self.accumulated_time -= Duration::from_secs_f32(target_delta);
        }
    }

    fn step(&mut self, delta: f32) {
        let substeps = 1;
        for _ in 0..substeps {
            self.world.update(delta / substeps as f32);
        }
        // println!("{:?}", self.world.softbodies()[0].points[0].pos());
        // self.world
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.renderer.set_camera(&self.camera);
        let mut canvas = Canvas::new();

        for softbody in self.world.softbodies() {
            // Only draw lines where there are points to connect
            if !softbody.points.len() > 1 {
                let mut prev_pos = softbody.points[softbody.points.len() - 1].position;
                for pointmass in &softbody.points {
                    let pos = pointmass.position;
                    canvas.draw_line(prev_pos, pos, 0.3, wgpu::Color::WHITE);

                    prev_pos = pos;
                }
            }

            for pointmass in &softbody.points {
                let pos = pointmass.position;
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

            // for segment in segments_from_points(&softbody.points) {
            //     canvas.draw_line(segment.start, segment.end, 0.3, wgpu::Color::BLUE)
            // }

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
