#![feature(get_many_mut)]

mod application;
mod rendering;
mod squish;

use log::error;
use rendering::camera::Camera2d;
use squish::entities::{PointMass, SoftBodyTemplate};
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

async fn run() {
    let mut world = squish::world::World::new();

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
    world.create_softbody(&template, (20.0, 20.0).into());

    world.create_softbody(
        &SoftBodyTemplate {
            points: vec![
                PointMass::new((0.0, 0.0).into(), 1.0),
                // PointMass::new((0.0, 40.0).into(), 1.0),
                PointMass::new((0.0, 10.0).into(), 1.0),
                PointMass::new((100.0, 10.0).into(), 1.0),
                PointMass::new((100.0, 0.0).into(), 1.0),
            ],
            springs: Vec::new(),
            is_dynamic: false,
        },
        (-40.0, -40.0).into(),
    );

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let renderer = rendering::renderer::Renderer::new(&window).await;
    let camera = Camera2d::new((0.0, 0.0).into(), 50.0);
    let mut application = application::Application::new(world, renderer, camera);

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            if !application.handle_input_event() {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        application.renderer.resize(*physical_size)
                    }
                    _ => {}
                }
            }
        }
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            application.update();
            match application.render() {
                Ok(_) => {}
                Err(e) => error!("{:?}", e),
            }
        }
        Event::MainEventsCleared => window.request_redraw(),
        _ => {}
    });
}

fn main() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .filter_module("wgpu_core", log::LevelFilter::Warn)
        .init();
    pollster::block_on(run());
}
