use crate::{
    app_state::AppState,
    resources::{GameConfig, ShipConfig, ShotConfig},
};
use bevy::prelude::*;
use bevy_asset_ron::RonAssetPlugin;

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(RonAssetPlugin::<GameConfig>::new(&["gameconfig"]))
            .add_system_set(
                SystemSet::on_enter(AppState::Loading).with_system(load_assets.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Loading)
                    .with_system(check_assets_have_loaded.system()),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Loading).with_system(process_assets.system()),
            );
    }
}

struct AssetHandles {
    handles: Vec<HandleUntyped>,
}

fn load_assets(mut commands: Commands, assets: Res<AssetServer>) {
    let mut handles = Vec::new();
    let configs = assets.load_folder("config").unwrap();
    let sprites = assets.load_folder("sprites").unwrap();

    for handle in configs {
        handles.push(handle.clone());
    }

    for handle in sprites {
        handles.push(handle.clone());
    }

    let asset_handles = AssetHandles { handles };
    commands.insert_resource(asset_handles);
}

fn check_assets_have_loaded(
    mut app_state: ResMut<State<AppState>>,
    asset_handles: Res<AssetHandles>,
    assets: Res<AssetServer>,
) {
    use bevy::asset::LoadState;
    match assets.get_group_load_state(asset_handles.handles.iter().map(|h| h.id)) {
        LoadState::Failed => {
            println!("ASSETS HAVE FAILED TO LOAD");
        }
        LoadState::Loaded => {
            app_state.set(AppState::InGame).unwrap();
        }
        _ => {}
    }
}

fn process_assets(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_configs: Res<Assets<GameConfig>>,
    assets: Res<AssetServer>,
) {
    let game_config_handle: Handle<GameConfig> = assets.load("config/config.gameconfig");
    if let Some(game_config) = game_configs.get(game_config_handle) {
        println!("WE IN HERE");
        commands.insert_resource(ShipConfig {
            acceleration: game_config.ship_acceleration,
            drag: game_config.ship_drag,
            firerate: game_config.ship_firerate,
        });
        commands.insert_resource(ShotConfig {
            speed: game_config.shot_speed,
            material: materials.add(assets.load(game_config.shot_material.as_str()).into()),
        });
    }
    println!("PROCESSING LOADED ASSETS");
}
