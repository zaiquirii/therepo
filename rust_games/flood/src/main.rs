mod components;
mod flood;
mod grid;
mod mobs;
mod movement;
mod towers;

use std::time::Duration;

use bevy::core_pipeline::core_2d;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::camera::{self, CameraProjection, CameraRenderGraph, ScalingMode, WindowOrigin};
use bevy::render::primitives::Frustum;
use bevy::render::view::VisibleEntities;
use bevy_prototype_lyon::prelude::*;

use flood::{
    mouse_record_system, setup_flood_demo_system, spawner_discharge_flood_system,
    update_flood_render_system, update_flood_system, FixedTime, MainCamera,
};
use iyes_loopless::prelude::*;
use mobs::spawn_mob;
use movement::movement_system;
use towers::towers::{spawn_tower, target_towers_system};

fn main() {
    let mut flood_fixed_update_stage = SystemStage::parallel();
    flood_fixed_update_stage
        .add_system(spawner_discharge_flood_system)
        .add_system(update_flood_system)
        .add_system(update_flood_render_system);

    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Flood".to_string(),
            width: 1000.0,
            height: 575.0,
            ..default()
        })
        .insert_resource(FixedTime)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_stage_before(
            CoreStage::Update,
            "flood_fixed_update",
            FixedTimestepStage::new(Duration::from_millis(50)).with_stage(flood_fixed_update_stage),
        )
        .add_startup_system(setup_system)
        .add_startup_system(setup_flood_demo_system)
        .add_system(rotate_system)
        .add_system(movement_system)
        .add_system(target_towers_system)
        .add_system(mouse_record_system)
        .run();
}

#[derive(Component, Deref)]
struct Rotate(f32);

fn setup_system(mut commands: Commands) {
    spawn_camera(&mut commands);
    // spawn_tower(&mut commands, Vec2::new(0.0, 0.0));
    // spawn_tower(&mut commands, Vec2::new(20.0, 20.0));
    // spawn_tower(&mut commands, Vec2::new(-30.0, -100.0));

    // spawn_mob(&mut commands, Vec2::new(-100.0, 0.0));
}

fn rotate_system(time: Res<Time>, mut query: Query<(&mut Transform, &Rotate)>) {
    for (mut transform, rotate) in query.iter_mut() {
        transform.rotate_z(time.delta_seconds() * rotate.to_radians())
    }
}

fn spawn_camera(commands: &mut Commands) {
    let far = 1000.0;
    let projection = OrthographicProjection {
        far,
        depth_calculation: camera::DepthCalculation::ZDifference,
        scaling_mode: ScalingMode::FixedVertical(45.0),
        window_origin: WindowOrigin::BottomLeft,
        ..default()
    };
    let transform = Transform::from_xyz(0.0, 0.0, far - 0.1);
    let view_projection = projection.get_projection_matrix() * transform.compute_matrix().inverse();
    let frustum = Frustum::from_view_projection(
        &view_projection,
        &transform.translation,
        &transform.back(),
        projection.far(),
    );

    // let mut camera_bundle = Camera2dBundle::default();
    // camera_bundle.transform = camera_bundle
    //     .transform
    //     .with_translation(Vec3::new(300.0, 300.0, 999.0));
    commands
        .spawn_bundle(Camera2dBundle {
            camera_render_graph: CameraRenderGraph::new(core_2d::graph::NAME),
            projection,
            visible_entities: VisibleEntities::default(),
            frustum,
            transform,
            global_transform: Default::default(),
            camera: Camera::default(),
            camera_2d: Camera2d::default(),
        })
        .insert(MainCamera);
}
