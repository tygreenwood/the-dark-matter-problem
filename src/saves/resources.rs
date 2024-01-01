use bevy::{
    ecs::{event::Event, system::Resource},
    math::Quat,
};

#[derive(Resource, Default)]
pub struct PositionSaveInformation {
    pub x: f32,
    pub y: f32,
}

#[derive(Resource, Default)]
pub struct WheelSaveInformation {
    pub rot: Quat,
}

#[derive(Event, Default)]
pub struct SaveGame;
