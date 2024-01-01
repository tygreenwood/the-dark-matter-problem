use bevy::prelude::*;
use bevy_renet::renet::ClientId;

#[derive(Component)]
pub struct Jump(pub f32);

#[derive(Component)]
pub struct Player {
    pub id: ClientId,
}

#[derive(Component)]
pub struct ControlledPlayer;

#[derive(Component)]
pub struct Cat;

#[derive(Resource)]
pub struct MyGamepad(pub Gamepad);
