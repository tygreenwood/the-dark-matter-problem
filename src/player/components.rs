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

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
