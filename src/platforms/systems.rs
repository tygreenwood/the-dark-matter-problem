use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::setup::configs::{WINDOW_BOTTOM_Y, WINDOW_WIDTH};

use super::{
    components::{Platform, PlatformBundle},
    configs::{COLOR_FLOOR, FLOOR_THICKNESS},
};

pub fn setup_entities(mut commands: Commands) {
    commands.spawn((
        PlatformBundle::new(Vec2::new(-100.0, 0.), Vec2::new(75.0, 200.0)),
        Platform,
    ));
    commands.spawn((
        PlatformBundle::new(Vec2::new(100.0, 0.), Vec2::new(50.0, 350.0)),
        Platform,
    ));
    commands.spawn((
        PlatformBundle::new(Vec2::new(350.0, 0.), Vec2::new(150.0, 250.0)),
        Platform,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: COLOR_FLOOR,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, WINDOW_BOTTOM_Y + FLOOR_THICKNESS / 2.0, -1.0),
                scale: Vec3::new(WINDOW_WIDTH * 10., FLOOR_THICKNESS, 1.0),
                ..default()
            },
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(0.5, 0.5),
        Platform,
    ));
}

pub fn cleanup(mut commands: Commands, query_platforms: Query<Entity, With<Platform>>) {
    for entity in &query_platforms {
        commands.entity(entity).despawn();
    }
}
