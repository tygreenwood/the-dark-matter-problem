use bevy::{
    prelude::{default, AssetServer, Assets, Commands, Query, Res, ResMut, Transform, Vec2, Vec3},
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    time::{Time, Timer, TimerMode},
};

use crate::{setup::WINDOW_BOTTOM_Y, platforms::FLOOR_THICKNESS};

use super::components::{AnimationIndices, AnimationTimer};

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

pub fn setup_mushrooms(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("shroom/Mushroom.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(13.0, 9.0), 31, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 0, last: 30 };
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            transform: Transform {translation: Vec3::new(0., WINDOW_BOTTOM_Y + FLOOR_THICKNESS*3., 0.), scale: Vec3::new(5., 5., 1.), ..default()},
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.03, TimerMode::Repeating)),
    ));
}