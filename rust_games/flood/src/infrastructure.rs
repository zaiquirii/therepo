use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_prototype_lyon::prelude::*;

use crate::{
    flood::MainCamera,
    logistics::{
        ecs::{LogisticsNode, LogisticsNodeRemoved},
        inventory::{Producer, Receiver, Resources},
    },
    ui, z_levels,
};

pub enum BuildingType {
    AmmoSupplier,
    LogisticsHub,
}

pub fn spawn_infrastructure(commands: &mut Commands, building_type: BuildingType, position: Vec2) {
    match building_type {
        BuildingType::LogisticsHub => spawn_logistics_node(commands, position),
        BuildingType::AmmoSupplier => spawn_ammo_supplier(commands, position),
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
        .insert(Receiver {
            requests: Resources { ammo: 4 },
            in_transit: Resources { ammo: 0 },
        });
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
        .insert(Producer {
            inventory: Resources { ammo: 1000 },
        });
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
                    commands.entity(entity).despawn();
                    ev_log_node_removed.send(LogisticsNodeRemoved(entity));
                    return;
                }
            }
            spawn_infrastructure(&mut commands, BuildingType::LogisticsHub, world_position);
        }
    }
}
