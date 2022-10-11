use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

use crate::mobs::Mob;

#[derive(Component, Deref, DerefMut)]
pub struct Range(f32);

#[derive(Component)]
pub struct Turret;

#[derive(Bundle)]
pub struct TowerBundle {
    #[bundle]
    shape_bundle: ShapeBundle,
    range: Range,
}

pub enum TowerType {
    Basic,
}

pub fn spawn_tower(commands: &mut Commands, position: Vec2) {
    let shape = shapes::Rectangle {
        extents: Vec2::new(20.0, 5.0),
        origin: RectangleOrigin::CustomCenter(Vec2::new(10.0, 0.0)),
    };

    commands
        .spawn_bundle(build_tower(TowerType::Basic, position))
        .with_children(|parent| {
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shape,
                    DrawMode::Fill(FillMode::color(Color::PINK)),
                    Transform::from_xyz(0.0, 0.0, 0.2),
                ))
                .insert(Turret);
        });
}

fn build_tower(tower_type: TowerType, position: Vec2) -> TowerBundle {
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(20.0),
        ..shapes::RegularPolygon::default()
    };

    TowerBundle {
        range: Range(10.0),
        shape_bundle: GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill(FillMode::color(Color::ORANGE)),
            Transform::from_xyz(position.x, position.y, 0.1),
        ),
    }
}

pub fn target_towers_system(
    mob_query: Query<&Transform, With<Mob>>,
    mut query: Query<(&mut Transform, &GlobalTransform), (With<Turret>, Without<Mob>)>,
) {
    // let mob = mob_query.single();
    // for (mut transform, global_transform) in &mut query {
    //     let delta = mob.translation.xy() - global_transform.translation().xy();
    //     let rotation = Vec2::X.angle_between(delta);
    //     transform.rotation = Quat::from_rotation_z(rotation);
    // }
}
