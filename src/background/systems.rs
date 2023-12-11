use bevy::{
    prelude::{default, AssetServer, Commands, GlobalTransform, Query, Res, Transform, Vec3, With},
    sprite::SpriteBundle,
    time::Time, // remove this if we get rid of the animated backgrounds
};

use crate::player::components::Player;

use super::components::Background;

pub fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("background/Background_2.png"),
            transform: Transform {
                translation: Vec3::new(0., 0., -3.),
                scale: Vec3::new(2., 2., 1.),
                ..default()
            },
            ..default()
        },
        //animation_indicies,
        //AnimationTimer(Timer::from_seconds(0.03, TimerMode::Repeating)),
        Background,
    ));
}
// ok listen this is in the middle of not working, switching the background
// to a non animated one in the meantime thx
// pub fn animate_sprite(
//     time: Res<Time>,
//     mut query: Query<(
//         &AnimationIndices,
//         &mut AnimationTimer,
//         &mut TextureAtlasSprite,
//     )>,
// ) {
//     for (indices, mut timer, mut sprite) in &mut query {
//         timer.tick(time.delta());
//         if timer.just_finished() {
//             sprite.index = if sprite.index == indices.last {
//                 indices.first
//             } else {
//                 sprite.index + 1
//             };
//         }
//     }
// }

pub fn move_background(
    query_player: Query<&GlobalTransform, With<Player>>,
    mut query_background: Query<&mut Transform, With<Background>>,
) {
    let player = query_player.single();
    let mut background = query_background.single_mut();

    background.translation.x = player.translation().x * 0.9;
}
