use crate::app_state::AppState;
use crate::player::PlayerConfig;
use bevy::prelude::*;
use bevy_asset_ron::RonAssetPlugin;

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(RonAssetPlugin::<PlayerConfig>::new(&["playerconfig"]))
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
    player_config: Handle<PlayerConfig>,
    handles: Vec<HandleUntyped>,
}

fn load_assets(mut commands: Commands, assets: Res<AssetServer>) {
    let mut handles = Vec::new();
    let player_config: Handle<PlayerConfig> = assets.load("config/config.playerconfig");
    handles.push(player_config.clone_untyped());

    let asset_handles = AssetHandles {
        player_config,
        handles,
    };
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
    player_configs: Res<Assets<PlayerConfig>>,
    handles: Res<AssetHandles>,
) {
    if let Some(player_config) = player_configs.get(&handles.player_config) {
        commands.insert_resource(player_config.clone());
    }
    println!("PROCESSING LOADED ASSETS");
}
