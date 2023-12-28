use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    platforms::configs::FLOOR_THICKNESS, player::components::Player,
    setup::configs::WINDOW_BOTTOM_Y,
};

use super::{
    components::{AnimationIndices, AnimationTimer, Mushroom},
    configs::MUSHROOM_PATH,
};

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<
        (
            &AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
        ),
        With<Mushroom>,
    >,
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
    let texture_handle = asset_server.load(MUSHROOM_PATH);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(13.0, 9.0), 31, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 0, last: 30 };
    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(0),
                transform: Transform {
                    translation: Vec3::new(0., WINDOW_BOTTOM_Y + FLOOR_THICKNESS * 3., 0.),
                    scale: Vec3::new(5., 5., 1.),
                    ..default()
                },
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.03, TimerMode::Repeating)),
            RigidBody::Fixed,
            Collider::cuboid(0.5, 0.5),
            Mushroom,
        ))
        .insert(Sensor)
        .insert(ActiveEvents::CONTACT_FORCE_EVENTS);
}

pub fn check_mushroom_hop(
    rapier_context: Res<RapierContext>,
    player_query: Query<Entity, With<Player>>,
    mushroom_query: Query<Entity, With<Mushroom>>,
) {
    let player = player_query.single();

    for mushroom in &mushroom_query {
        if rapier_context.intersection_pair(player, mushroom) == Some(true) {
            println!("intersecting!!")
        }
    }
}

pub fn cleanup(mut commands: Commands, query_mushrooms: Query<Entity, With<Mushroom>>) {
    for entity in &query_mushrooms {
        commands.entity(entity).despawn();
    }
}
