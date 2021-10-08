use bevy::prelude::*;

use crate::app_state::AppState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame).with_system(spawn_enemy.system()),
        )
        .insert_resource(SpawnTimer(Timer::from_seconds(3.0, false)));
    }
}

pub struct Enemy;

pub struct EnemyConfig {}

struct SpawnTimer(Timer);

fn spawn_enemy(
    mut commands: Commands,
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    assets: Res<AssetServer>,
    mut spawn_timer: ResMut<SpawnTimer>,
) {
    if spawn_timer.0.tick(time.delta()).just_finished() {
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            material: materials.add(assets.load("sprites/circle.png").into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        });
    }
}
