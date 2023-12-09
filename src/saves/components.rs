use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)] // this tells the reflect derive to also reflect component behaviors
pub struct PositionSave {
    pub x: f32,
    pub y: f32,
}

#[derive(Event, Default)]
pub struct SaveEvent;
