use bevy::prelude::*;

use crate::app_state::AppState;
use crate::physics::Kinematics;
use crate::resources::{ShipConfig, ShotConfig};

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(move_ship.system())
                .with_system(fire_shots.system()),
        );
    }
}

#[derive(Default)]
pub struct ShipInput {
    pub direction: Vec2,
    pub shot_angle: f32,
    pub shooting: bool,
}

#[derive(Default)]
pub struct Ship {
    last_shot_at: f64,
}

#[derive(Bundle, Default)]
pub struct ShipBundle {
    ship_input: ShipInput,
    kinematics: Kinematics,
    ship: Ship,

    #[bundle]
    sprite: SpriteBundle,
}

impl ShipBundle {
    pub fn new(ship_config: ShipConfig, texture: Handle<ColorMaterial>) -> Self {
        ShipBundle {
            ship_input: ShipInput::default(),
            kinematics: Kinematics {
                drag: ship_config.drag,
                ..Default::default()
            },
            sprite: SpriteBundle {
                sprite: Sprite::new(Vec2::new(10.0, 11.0)),
                material: texture,
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

fn move_ship(mut query: Query<(&ShipInput, &mut Kinematics, &mut Transform)>) {
    for (ship_input, mut physics, mut transform) in query.iter_mut() {
        physics.acceleration = ship_input.direction.clamp_length_max(1.0) * 1000.0;
        transform.rotation = Quat::from_rotation_z(ship_input.shot_angle);
    }
}

fn fire_shots(
    mut commands: Commands,
    time: Res<Time>,
    ship_config: Res<ShipConfig>,
    shot_config: Res<ShotConfig>,
    mut query: Query<(&ShipInput, &mut Ship, &Transform)>,
) {
    let shot_delay = 1.0 / ship_config.firerate as f64;
    for (input, mut ship, transform) in query.iter_mut() {
        if input.shooting && ship.last_shot_at + shot_delay < time.seconds_since_startup() {
            // FIRE A SHOT
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite::new(Vec2::new(7.0, 7.0)),
                    material: shot_config.material.clone(),
                    transform: transform.clone(),
                    ..Default::default()
                })
                .insert(Kinematics {
                    velocity: Vec2::new(-input.shot_angle.sin(), input.shot_angle.cos())
                        * shot_config.speed,
                    ..Default::default()
                });
            ship.last_shot_at = time.seconds_since_startup();
        }
    }
}
