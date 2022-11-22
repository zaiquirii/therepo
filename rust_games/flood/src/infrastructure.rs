use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_prototype_lyon::prelude::*;

use crate::{
    flood::MainCamera,
    logistics::{
        ecs::{LogisticsNode, LogisticsNodeRemoved},
        inventory::{EnergyConsumer, EnergySupplier},
    },
    towers::towers::spawn_tower,
    ui, z_levels,
};

pub enum BuildingType {
    AmmoSupplier,
    LogisticsHub,
    Turret,
}

pub fn spawn_infrastructure(commands: &mut Commands, building_type: BuildingType, position: Vec2) {
    match building_type {
        BuildingType::LogisticsHub => spawn_logistics_node(commands, position),
        BuildingType::AmmoSupplier => spawn_ammo_supplier(commands, position),
        BuildingType::Turret => spawn_tower(commands, position),
    }
}

fn spawn_logistics_node(commands: &mut Commands, position: Vec2) {
    let shape = shapes::Rectangle {
        extents: Vec2::new(2.0, 2.0),
        origin: RectangleOrigin::BottomLeft,
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill(FillMode::color(Color::YELLOW)),
            Transform::from_xyz(position.x, position.y, z_levels::INFRASTRUCTURE),
        ))
        .insert(LogisticsNode::new(20.0))
        .insert(EnergyConsumer::new(4, 1));
}

fn spawn_ammo_supplier(commands: &mut Commands, position: Vec2) {
    let shape = shapes::Rectangle {
        extents: Vec2::new(2.0, 2.0),
        origin: RectangleOrigin::BottomLeft,
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill(FillMode::color(Color::ORANGE_RED)),
            Transform::from_xyz(position.x, position.y, z_levels::INFRASTRUCTURE),
        ))
        .insert(LogisticsNode::new(20.0))
        .insert(EnergySupplier::new(1000));
}

pub fn basic_click_for_infrastructure_system(
    mut commands: Commands,
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    mut ev_log_node_removed: EventWriter<LogisticsNodeRemoved>,
    q_log_nodes: Query<(Entity, &LogisticsNode, &GlobalTransform)>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = q_camera.single();
        if let Some(pointer_info) = ui::get_pointer_info(camera, camera_transform, &windows) {
            let pos = pointer_info.position.grid_position;
            let world_position = Vec2::new(pos.x as f32, pos.y as f32);

            for (entity, _node, transform) in q_log_nodes.iter() {
                if (world_position - transform.translation().xy()).length() < 5.0 {
                    commands.entity(entity).despawn_recursive();
                    ev_log_node_removed.send(LogisticsNodeRemoved(entity));
                    return;
                }
            }
            spawn_infrastructure(&mut commands, BuildingType::Turret, world_position);
        }
    }
}
