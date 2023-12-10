use bevy::{
    prelude::{default, AssetServer, Commands, GlobalTransform, Query, Res, Transform, Vec3, With},
    sprite::SpriteBundle,
};

use crate::player::components::Player;

use super::components::Background;

pub fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("BackgroundTest1.png"),
            transform: Transform {
                translation: Vec3::new(0., 0., -2.),
                scale: Vec3::new(18., 18., 1.),
                ..default()
            },
            ..default()
        },
        Background,
    ));
}

pub fn move_background(
    query_player: Query<&GlobalTransform, With<Player>>,
    mut query_background: Query<&mut Transform, With<Background>>,
) {
    let player = query_player.single();
    let mut background = query_background.single_mut();

    background.translation.x = player.translation().x * 0.9;
}
