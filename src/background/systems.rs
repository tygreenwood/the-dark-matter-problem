use bevy::{
    prelude::{default, AssetServer, Commands, Res, Transform, Vec3},
    sprite::SpriteBundle,
};

use super::components::Background;

pub fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("BackgroundTest1.png"),
            transform: Transform {
                translation: Vec3::new(0., 0., -1.),
                scale: Vec3::new(18., 18., 1.),
                ..default()
            },
            ..default()
        },
        Background,
    ));
}
