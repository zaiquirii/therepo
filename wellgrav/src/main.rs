mod app_state;
mod asset_loading;
mod components;
mod physics;
mod player;
mod ships;
mod resources;
mod enemies;

use bevy::input::system::exit_on_esc_system;
use bevy::prelude::*;
use enemies::EnemyPlugin;
use resources::ShipConfig;
use ships::ShipPlugin;

use crate::app_state::AppState;
use crate::player::{MainCamera, Player};
use crate::ships::{ShipBundle, ShipInput};
use asset_loading::AssetLoadingPlugin;
use player::PlayerPlugin;

fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ship_config: Res<ShipConfig>,
) {
    let texture_handle = asset_server.load("sprites/triangle.png");
    let material = materials.add(texture_handle.into());

    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    commands
        .spawn_bundle(ShipBundle::new(*ship_config, material))
        .insert(Player);
    println!("CREATING PLAYER");
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_prototype_debug_lines::DebugLinesPlugin)
        .add_plugin(physics::PhysicsPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(ShipPlugin)
        .add_plugin(AssetLoadingPlugin)
        .add_state(AppState::Loading)
        .add_system(exit_on_esc_system.system())
        .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup_scene.system()))
        .run();
}
