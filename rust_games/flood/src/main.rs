mod components;
mod flood;
mod grid;
mod mobs;
mod movement;
mod towers;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use flood::{setup_flood_demo_system, update_flood_render_system, update_flood_system};
use mobs::spawn_mob;
use movement::movement_system;
use towers::towers::{spawn_tower, target_towers_system};

// use towers::towers::;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)
        .add_startup_system(setup_flood_demo_system)
        .add_system(rotate_system)
        .add_system(movement_system)
        .add_system(target_towers_system)
        .add_system(update_flood_system)
        .add_system(update_flood_render_system)
        .run();
}

#[derive(Component, Deref)]
struct Rotate(f32);

fn setup_system(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
    spawn_tower(&mut commands, Vec2::new(0.0, 0.0));
    spawn_tower(&mut commands, Vec2::new(20.0, 20.0));
    spawn_tower(&mut commands, Vec2::new(-30.0, -100.0));

    spawn_mob(&mut commands, Vec2::new(-100.0, 0.0));
}

fn rotate_system(time: Res<Time>, mut query: Query<(&mut Transform, &Rotate)>) {
    for (mut transform, rotate) in query.iter_mut() {
        transform.rotate_z(time.delta_seconds() * rotate.to_radians())
    }
}
