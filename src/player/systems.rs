use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    saves::resources::PositionSaveInformation,
    setup::configs::{WINDOW_BOTTOM_Y, WINDOW_LEFT_X},
};

use super::{
    components::{AnimationIndices, AnimationTimer, Cat, Jump, Player},
    configs::{CAT_ANIMATION_PATH, PLAYER_RUNNING_ANIMATION_PATH},
};

const PLAYER_VELOCITY_X: f32 = 400.0;

const PLAYER_VELOCITY_Y: f32 = 850.0;

const MAX_JUMP_HEIGHT: f32 = 230.0;

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let player_texture_handle = asset_server.load(PLAYER_RUNNING_ANIMATION_PATH);
    let player_texture_atlas = TextureAtlas::from_grid(
        player_texture_handle,
        Vec2::new(19.0, 32.0),
        7,
        1,
        None,
        None,
    );
    let player_texture_atlas_handle = texture_atlases.add(player_texture_atlas);
    let player_animation_indices = AnimationIndices { first: 0, last: 6 };

    let cat_texture_handle = asset_server.load(CAT_ANIMATION_PATH);
    let cat_texture_atlas =
        TextureAtlas::from_grid(cat_texture_handle, Vec2::new(25.0, 16.0), 3, 1, None, None);
    let cat_texture_atlas_handle = texture_atlases.add(cat_texture_atlas);
    let cat_animation_indices = AnimationIndices { first: 0, last: 2 };

    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: player_texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(WINDOW_LEFT_X + 100.0, WINDOW_BOTTOM_Y + 300.0, 0.0),
                    scale: Vec3::new(5., 5., 1.),
                    ..default()
                },
                ..default()
            },
            Player,
            RigidBody::KinematicPositionBased,
            Collider::cuboid(9.5, 15.0),
            KinematicCharacterController::default(),
            player_animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteSheetBundle {
                    texture_atlas: cat_texture_atlas_handle,
                    transform: Transform {
                        translation: Vec3::new(-20., -11., 0.),
                        scale: Vec3::new(0.5, 0.5, 1.),
                        ..default()
                    },
                    ..default()
                },
                Cat,
                cat_animation_indices,
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            ));
        });
}

pub fn movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut KinematicCharacterController, &GlobalTransform)>,
    mut query_player_flip: Query<&mut TextureAtlasSprite, (With<Player>, Without<Cat>)>,
    mut query_cat_flip: Query<
        (&mut TextureAtlasSprite, &mut Transform),
        (With<Cat>, Without<Player>),
    >,
    mut pos_save: ResMut<PositionSaveInformation>,
) {
    let (mut player, player_pos) = query.single_mut();

    let mut movement = 0.0;

    let mut player_flip = query_player_flip.single_mut();
    let (mut cat_flip, mut cat_position) = query_cat_flip.single_mut();

    if input.pressed(KeyCode::Right) {
        movement += time.delta_seconds() * PLAYER_VELOCITY_X;
        if player_flip.flip_x {
            player_flip.flip_x = false;
            cat_flip.flip_x = false;
            cat_position.translation.x = -20.0;
        }
    }

    if input.pressed(KeyCode::Left) {
        movement += time.delta_seconds() * PLAYER_VELOCITY_X * -1.0;
        if !player_flip.flip_x {
            player_flip.flip_x = true;
            cat_flip.flip_x = true;
            cat_position.translation.x = 20.0;
        }
    }

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(movement, vec.y)), // update if it already exists
        None => player.translation = Some(Vec2::new(movement, 0.0)),
    }

    pos_save.x = player_pos.translation().x;
    pos_save.y = player_pos.translation().y;
}

pub fn jump(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    query: Query<
        (Entity, &KinematicCharacterControllerOutput),
        (With<KinematicCharacterController>, Without<Jump>),
    >,
) {
    if query.is_empty() {
        return;
    }

    let (player, output) = query.single();

    if input.pressed(KeyCode::Up) && output.grounded {
        commands.entity(player).insert(Jump(0.0));
    }
}

pub fn rise(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut KinematicCharacterController, &mut Jump)>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, mut player, mut jump) = query.single_mut();

    let mut movement = time.delta().as_secs_f32() * PLAYER_VELOCITY_Y;

    if movement + jump.0 >= MAX_JUMP_HEIGHT {
        movement = MAX_JUMP_HEIGHT - jump.0;
        commands.entity(entity).remove::<Jump>();
    }

    jump.0 += movement;

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
        None => player.translation = Some(Vec2::new(0.0, movement)),
    }
}

pub fn fall(
    time: Res<Time>,
    mut query: Query<(&mut KinematicCharacterController, &mut Transform), Without<Jump>>,
) {
    if query.is_empty() {
        return;
    }

    let (mut player, mut pos) = query.single_mut();

    // I am using two-thirds of the Y-velocity since I want the character to fall slower than it rises
    let movement = time.delta().as_secs_f32() * (PLAYER_VELOCITY_Y / 1.5) * -1.0;

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
        None => player.translation = Some(Vec2::new(0.0, movement)),
    }

    if pos.translation.y < WINDOW_BOTTOM_Y - 10.0 {
        pos.translation = Vec3::new(WINDOW_LEFT_X + 100.0, WINDOW_BOTTOM_Y + 300.0, 0.0);
    }
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query_player: Query<
        (
            &AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
        ),
        (With<Player>, Without<Cat>),
    >,
    mut query_cat: Query<
        (
            &AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
        ),
        (With<Cat>, Without<Player>),
    >,
    input: Res<Input<KeyCode>>,
) {
    for (indices, mut timer, mut sprite) in &mut query_player {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index != indices.last
                && (sprite.index != indices.first
                    || (input.pressed(KeyCode::Left) || input.pressed(KeyCode::Right)))
            {
                sprite.index + 1
            } else {
                indices.first
            };
        }
    }

    for (indices, mut timer, mut sprite) in &mut query_cat {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index != indices.last
                && (sprite.index != indices.first
                    || (input.pressed(KeyCode::Left) || input.pressed(KeyCode::Right)))
            {
                sprite.index + 1
            } else {
                indices.first
            };
        }
    }
}

pub fn cleanup(
    mut commands: Commands,
    query_sprites: Query<Entity, Or<(With<Player>, With<Cat>)>>,
) {
    for entity in &query_sprites {
        commands.entity(entity).despawn();
    }
}
