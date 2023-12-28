use bevy::prelude::*;

#[derive(Component)]
pub struct Jump(pub f32);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Cat;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
