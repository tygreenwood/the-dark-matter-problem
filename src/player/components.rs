use bevy::prelude::*;
use bevy_renet::renet::ClientId;

#[derive(Component)]
pub struct Jump {
    pub time: f32,
    pub initial_velocity: f32,
}

#[derive(Component)]
pub struct Player {
    pub id: ClientId,
}

#[derive(Component)]
pub struct ControlledPlayer;

#[derive(Component)]
pub struct Cat;

#[derive(Component)]
pub struct Dash {
    pub velocity: f32,
    pub timer: Timer,
}

#[derive(Component)]
pub struct DashCooldown(pub Timer);

#[derive(Resource)]
pub struct MyGamepad(pub Gamepad);
