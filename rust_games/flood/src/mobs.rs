use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

use crate::movement::Velocity;

#[derive(Component)]
pub struct Mob;

pub fn spawn_mob(commands: &mut Commands, position: Vec2) {
    let shape = shapes::Circle {
        radius: 10.0,
        center: Vec2::ZERO,
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill(FillMode::color(Color::GREEN)),
            Transform::from_xyz(position.x, position.y, 0.0),
        ))
        .insert(Mob)
        .insert(Velocity(Vec2::new(10.0, 0.0)));
}
