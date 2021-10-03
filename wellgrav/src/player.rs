use bevy::math::Vec3Swizzles;
use bevy::math::Vec4Swizzles;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;

use crate::app_state::AppState;
use crate::physics;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(process_player_input.system())
                .with_system(move_player_ship.system()),
        );
    }
}

#[derive(serde::Deserialize, TypeUuid, Debug, Copy, Clone)]
#[uuid = "1df82c01-9c71-4fa8-adc4-78c5822268f8"]
pub struct PlayerConfig {
    pub acceleration: f32,
    pub drag: f32,
}

pub struct Player;

pub struct MainCamera;

pub struct ShipInput {
    pub direction: Vec2,
    pub shot_angle: f32,
    pub shooting: bool,
}

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
}

fn move_player_ship(
    mut query: Query<(
        &Player,
        &ShipInput,
        &mut physics::Kinematics,
        &mut Transform,
    )>,
) {
    let (_player, ship_input, mut physics, mut transform) =
        query.single_mut().expect("Only one player should exist");
    physics.acceleration = ship_input.direction.clamp_length_max(1.0) * 1000.0;
    transform.rotation = Quat::from_rotation_z(ship_input.shot_angle);
}
