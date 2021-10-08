use bevy::prelude::*;
use bevy::reflect::TypeUuid;

#[derive(serde::Deserialize, TypeUuid, Debug, Clone)]
#[uuid = "1df82c01-9c71-4fa8-adc4-78c5822268f8"]
pub struct GameConfig {
    pub ship_acceleration: f32,
    pub ship_drag: f32,
    pub ship_firerate: u32,
    pub shot_speed: f32,
    pub shot_material: String,
}

#[derive(Clone)]
pub struct ShotConfig {
    pub speed: f32,
    pub material: Handle<ColorMaterial>,
}

#[derive(Clone, Copy)]
pub struct ShipConfig {
    pub acceleration: f32,
    pub drag: f32,
    pub firerate: u32,
}
