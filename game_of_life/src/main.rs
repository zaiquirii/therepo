mod gol;

use log::{debug, error};
use winit::{
    dpi::LogicalSize,
    event::{Event, MouseButton, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 30;
const HEIGHT: u32 = 30;
fn main() -> Result<(), pixels::Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window_size = LogicalSize::new(WIDTH as f64 * 8.0, HEIGHT as f64 * 8.0);
    let window = WindowBuilder::new()
        .with_inner_size(window_size)
        .with_min_inner_size(window_size)
        .build(&event_loop)
        .unwrap();

    let mut simulation = gol::Simulation::new(WIDTH as usize, HEIGHT as usize);
    simulation.toggle(20, 20);
    simulation.toggle(21, 20);
    simulation.toggle(22, 20);
    simulation.toggle(23, 20);
    simulation.toggle(24, 20);
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture =
            pixels::SurfaceTexture::new(window_size.width, window_size.height, &window);
        pixels::Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            simulation.draw(pixels.get_frame_mut());
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.mouse_pressed(0) {
                if let Some(mouse_pos) = input.mouse() {
                    let x = (mouse_pos.0 / window.inner_size().width as f32) * WIDTH as f32;
                    let y = (mouse_pos.1 / window.inner_size().height as f32) * HEIGHT as f32;
                    eprintln!("mouse click at: {:?}, x: {}, y: {}", mouse_pos, x, y);
                    simulation.toggle(x as usize, y as usize);
                }
            }

            if input.key_pressed(VirtualKeyCode::Space) || input.key_held(VirtualKeyCode::S) {
                simulation.step();
            }

            if input.key_pressed(VirtualKeyCode::D) {
                simulation.swap();
            }

            window.request_redraw();
        }
    });
}
