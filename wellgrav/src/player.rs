use bevy::math::Vec3Swizzles;
use bevy::math::Vec4Swizzles;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;

use crate::app_state::AppState;
use crate::physics;
use crate::ships;
use crate::ships::ShipInput;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(process_player_input.system())
        );
    }
}
pub struct Player;

pub struct MainCamera;

fn process_player_input(
    buttons: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut q_player: Query<(&Player, &Transform, &mut ShipInput)>,
    q_camera: Query<&Transform, With<MainCamera>>,
) {
    let (_player, transform, mut ship_input) = q_player
        .single_mut()
        .expect("There should always be exactly one player in the game.");

    let left_pressed = buttons.pressed(KeyCode::A);
    let right_pressed = buttons.pressed(KeyCode::D);
    let up_pressed = buttons.pressed(KeyCode::W);
    let down_pressed = buttons.pressed(KeyCode::S);

    ship_input.direction.x =
        if left_pressed { -1.0 } else { 0.0 } + if right_pressed { 1.0 } else { 0.0 };
    ship_input.direction.y =
        if down_pressed { -1.0 } else { 0.0 } + if up_pressed { 1.0 } else { 0.0 };

    // Where is the mouse?
    let window = windows.get_primary().unwrap();
    if let Some(cursor_pos) = window.cursor_position() {
        let size = Vec2::new(window.width() as f32, window.height() as f32);
        let working_pos = cursor_pos - (size / 2.0);
        let camera_transform = q_camera.single().unwrap();
        let world_position =
            camera_transform.compute_matrix() * working_pos.extend(0.0).extend(1.0);

        let player_pos: Vec2 = transform.translation.xy();
        let mouse_vec = world_position.xy() - player_pos;
        let origin_vec = Vec2::Y;
        let angle = origin_vec.angle_between(mouse_vec);
        ship_input.shot_angle = angle;
    }

    // ARE WE FIRING?
    ship_input.shooting = mouse.pressed(MouseButton::Left);
}
