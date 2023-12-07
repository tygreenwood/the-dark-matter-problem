use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody};

use crate::setup::{resources::DisplayScale, WINDOW_BOTTOM_Y, WINDOW_WIDTH};

use super::{components::PlatformBundle, COLOR_FLOOR, FLOOR_THICKNESS};

pub fn setup_entities(mut commands: Commands, scale: Res<DisplayScale>) {
    commands.spawn(PlatformBundle::new(
        Vec2::new(-100.0, 0.),
        Vec2::new(75.0, 200.0),
    ));
    commands.spawn(PlatformBundle::new(
        Vec2::new(100.0, 0.),
        Vec2::new(50.0, 350.0),
    ));
    commands.spawn(PlatformBundle::new(
        Vec2::new(350.0, 0.),
        Vec2::new(150.0, 250.0),
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: COLOR_FLOOR,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(
                    0.0,
                    (WINDOW_BOTTOM_Y * scale.0) + (FLOOR_THICKNESS * scale.0) / 2.0,
                    0.0,
                ),
                scale: Vec3::new(WINDOW_WIDTH * scale.0, FLOOR_THICKNESS * scale.0, 1.0),
                ..default()
            },
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(0.5, 0.5),
    ));
}
