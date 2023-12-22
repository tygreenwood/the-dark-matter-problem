use bevy::{ecs::system::Resource, math::Quat};

#[derive(Resource, Default)]
pub struct PositionSaveInformation {
    pub x: f32,
    pub y: f32,
}

#[derive(Resource, Default)]
pub struct WheelSaveInformation {
    pub rot: Quat,
}

#[derive(Resource)]
pub struct SaveGame {
    pub save: bool,
}

impl Default for SaveGame {
    fn default() -> SaveGame {
        SaveGame { save: false }
    }
}
