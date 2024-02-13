mod simulation;
mod time;
mod quadtree;

use macroquad::prelude::*;
use crate::quadtree::{NodeId, Quadtree};
use crate::simulation::{Impulser, Simulation};


#[macroquad::main("nbody_simulation")]
async fn main() {
    request_new_screen_size(480.0 * 2.0, 360.0 * 2.0);
    next_frame().await;

    let delta_t = 1.0 / 60.0;
    let mut simulation = Simulation::new(3000, Rect::new(-screen_width() / 2.0, -screen_height() / 2.0, screen_width(), screen_height()));
    let mut show_grid = true;
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::G) {
            show_grid = !show_grid;
        }

        let offset = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = mouse_position_local();
            let pos = Vec2::new(mouse_pos.x * screen_width() / 2.0, mouse_pos.y * screen_height() / 2.0);
            simulation.add_impulser(Impulser::new(pos, 10000.0, 5000.0));
        }


        clear_background(BLACK);
        // if is_key_pressed(KeyCode::Space) {
        //     simulation.tick(delta_t);
        //     for b in simulation.bodies() {
        //         println!("body {:?}", b.pos)
        //     }
        // }
        simulation.tick(delta_t);
        for b in simulation.bodies() {
            draw_circle(b.pos.x + offset.x, b.pos.y + offset.y, b.size, RED);
        }

        if let Some(qt) = simulation.qt() {
            if show_grid {
                draw_quadtree_node(qt, qt.root, offset)
            }
        }

        next_frame().await
    }
}

fn draw_quadtree_node<T>(qt: &Quadtree<T>, node_id: NodeId, offset: Vec2) {
    let node = &qt.nodes[node_id.index];
    match node.children {
        Some(children) => {
            for child in children {
                draw_quadtree_node(qt, child, offset);
            }
        }
        None => {
            let b = node.bounds;
            draw_rectangle_lines(b.x + offset.x, b.y + offset.y, b.w, b.h, 1.0, GREEN)
        }
    }
}
