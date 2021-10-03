mod app_state;
mod asset_loading;
mod components;
mod physics;
mod player;

use bevy::input::system::exit_on_esc_system;
use bevy::prelude::*;

use crate::app_state::AppState;
use crate::player::{MainCamera, Player, ShipInput};
use asset_loading::AssetLoadingPlugin;
use player::{PlayerConfig, PlayerPlugin};

fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_config: Res<PlayerConfig>,
) {
    let texture_handle = asset_server.load("sprites/triangle.png");
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite::new(Vec2::new(10.0, 11.0)),
            material: materials.add(texture_handle.into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(ShipInput {
            direction: Vec2::new(0.0, 0.0),
            shot_angle: 0.0,
            shooting: false,
        })
        .insert(physics::Kinematics {
            velocity: Vec2::default(),
            acceleration: Vec2::default(),
            drag: player_config.drag,
        })
        .insert(Player);
    println!("CREATING PLAYER");
}


fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(physics::PhysicsPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(AssetLoadingPlugin)
        .add_state(AppState::Loading)
        .add_system(exit_on_esc_system.system())
        .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup_scene.system()))
        .run();
}
