use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(move_kinematics.system());
    }
}

fn move_kinematics(time: Res<Time>, mut query: Query<(&mut Transform, &mut Kinematics)>) {
    for (mut transform, mut kinematics) in query.iter_mut() {
        let acceleration = kinematics.acceleration + (kinematics.velocity * -kinematics.drag);
        kinematics.velocity += acceleration * time.delta_seconds();
        transform.translation.x += kinematics.velocity.x * time.delta_seconds();
        transform.translation.y += kinematics.velocity.y * time.delta_seconds();
    }
}

pub struct Kinematics {
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub drag: f32,
}
