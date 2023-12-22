use bevy::prelude::*;

use crate::player::components::Player;

use super::{
    components::{AnimationIndices, AnimationTimer, HorizonBackground, SpaceBackground},
    configs::{HORIZON_PATH, SPACE_PLANET_ANIMATION_PATH},
};

pub fn setup_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load(SPACE_PLANET_ANIMATION_PATH);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(1600.0, 950.0), 8, 8, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 0, last: 59 };
    commands
        .spawn((
            SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    flip_x: false,
                    ..default()
                },
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(0., 0., -4.),
                    scale: Vec3::new(2., 2., 1.),
                    ..default()
                },
                ..default()
            },
            SpaceBackground,
            animation_indices,
            AnimationTimer {
                timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                flip: false,
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: asset_server.load(HORIZON_PATH),
                    transform: Transform {
                        translation: Vec3::new(0., 0., 1.),
                        scale: Vec3::new(2., 1., 1.),
                        ..default()
                    },
                    ..default()
                },
                HorizonBackground,
            ));
        });
}

pub fn animate_background(
    time: Res<Time>,
    mut query: Query<
        (
            &AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
        ),
        With<SpaceBackground>,
    >,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.timer.tick(time.delta());
        if timer.timer.just_finished() {
            if !timer.flip {
                if sprite.index == indices.last {
                    timer.flip = true;
                    sprite.index -= 1;
                } else {
                    sprite.index += 1;
                }
            } else {
                if sprite.index == indices.first {
                    timer.flip = false;
                    sprite.index += 1;
                } else {
                    sprite.index -= 1;
                }
            };
        }
    }
}

pub fn move_background(
    query_player: Query<&GlobalTransform, With<Player>>,
    mut query_space_background: Query<
        &mut Transform,
        (With<SpaceBackground>, Without<HorizonBackground>),
    >,
    mut query_horizon_background: Query<
        &mut Transform,
        (With<HorizonBackground>, Without<SpaceBackground>),
    >,
) {
    let player = query_player.single();
    let mut space_background = query_space_background.single_mut();
    let mut horizon_background = query_horizon_background.single_mut();

    space_background.translation.x = player.translation().x * 0.9;
    horizon_background.translation.x = -player.translation().x * 0.1;
}
