use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)] // this tells the reflect derive to also reflect component behaviors
pub struct PositionSaveComponent {
    pub x: f32,
    pub y: f32,
}

#[derive(Event, Default)]
pub struct SaveEvent;

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct PositionSaveInformation {
    pub x: f32,
    pub y: f32,
}
