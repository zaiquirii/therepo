use bevy::prelude::*;

#[derive(Component, Deref)]
pub struct Velocity(pub Vec2);

pub fn movement_system(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation += (velocity.0 * time.delta_seconds()).extend(0.0);
    }
}
