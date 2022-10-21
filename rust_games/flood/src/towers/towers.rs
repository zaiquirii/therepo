use std::default;

use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

use crate::{
    flood::Flood,
    logistics::{ecs::LogisticsNode, inventory::EnergyConsumer},
    z_levels,
};

#[derive(Component)]
pub struct Health {
    current: u32,
    max: u32,
}

impl Health {
    pub fn new(max_health: u32) -> Self {
        Health {
            current: max_health,
            max: max_health,
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Platform {
    extents: UVec2,
}

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

#[derive(Component, Default)]
pub struct TowerTargetingSystem {
    pub range: i32,
    pub target_selection: TargetSelection,
    pub fire_timer: Timer,
}

impl TowerTargetingSystem {
    pub fn new(range: i32, firerate: f32, target_selection: TargetSelection) -> Self {
        TowerTargetingSystem {
            range,
            target_selection,
            fire_timer: Timer::from_seconds(firerate, false),
        }
    }
}

#[derive(Default)]
pub enum TargetSelection {
    #[default]
    Closest,
    Deepest,
}

pub enum TowerType {
    Basic,
}

pub fn spawn_tower(commands: &mut Commands, position: Vec2) {
    let shape = shapes::Rectangle {
        extents: Vec2::new(1.0, 3.0),
        origin: RectangleOrigin::CustomCenter(Vec2::new(0.0, 1.5)),
    };

    commands
        .spawn_bundle(build_tower(TowerType::Basic, position))
        .insert(TowerTargetingSystem::new(
            10,
            0.01,
            TargetSelection::Closest,
        ))
        .insert(LogisticsNode::new(20.0))
        .insert(EnergyConsumer::new(10, 4))
        // .insert(Health::new(100))
        // .insert(Platform {
        //     extents: UVec2::new(3, 3),
        // })
        .with_children(|parent| {
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shape,
                    DrawMode::Fill(FillMode::color(Color::PINK)),
                    Transform::from_xyz(1.5, 1.5, z_levels::INFRASTRUCTURE),
                ))
                .insert(Turret);
        });
}

fn build_tower(tower_type: TowerType, position: Vec2) -> TowerBundle {
    let shape = shapes::Rectangle {
        extents: Vec2::new(3.0, 3.0),
        origin: RectangleOrigin::BottomLeft,
    };

    TowerBundle {
        range: Range(5.0),
        shape_bundle: GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill(FillMode::color(Color::ORANGE)),
            Transform::from_xyz(position.x, position.y, 3.0),
        ),
    }
}

pub fn target_towers_system(
    time: Res<Time>,
    mut flood: ResMut<Flood>,
    mut q_turrets: Query<&mut Transform, With<Turret>>,
    mut q_towers: Query<
        (
            &Children,
            &mut Transform,
            &mut TowerTargetingSystem,
            &mut EnergyConsumer,
        ),
        Without<Turret>,
    >,
) {
    q_towers.for_each_mut(
        |(children, mut transform, mut tower_targeting_system, mut energy)| {
            let range = tower_targeting_system.range;
            let position = transform.translation.xy();
            tower_targeting_system.fire_timer.tick(time.delta());
            if let Some(grid_point) = flood.closest_flood(position, range) {
                // Rotate turret
                let delta = Vec2::new(
                    grid_point.x as f32 - position.x,
                    grid_point.y as f32 - position.y,
                );
                let rotation = Vec2::Y.angle_between(delta);

                for &child in children.iter() {
                    if let Ok(mut child_transform) = q_turrets.get_mut(child) {
                        child_transform.rotation = Quat::from_rotation_z(rotation);
                    }
                }

                // NOW FIRE
                let charge_per_shot = 1;
                if tower_targeting_system.fire_timer.finished() {
                    // if converter is empty
                    if energy.consume(charge_per_shot) {
                        tower_targeting_system.fire_timer.reset();
                        for i in -2..2 {
                            for j in -2..2 {
                                let x = grid_point.x + i;
                                let y = grid_point.y + j;
                                flood.set_flood_height(x as usize, y as usize, 0.0);
                            }
                        }
                    }
                }
            }
        },
    )
}
