use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody};

use crate::setup::WINDOW_BOTTOM_Y;

use super::COLOR_PLATFORM;

#[derive(Bundle)]
pub struct PlatformBundle {
    sprite_bundle: SpriteBundle,
    body: RigidBody,
    collider: Collider,
}

impl PlatformBundle {
    pub fn new(pos: Vec2, size: Vec2) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: COLOR_PLATFORM,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(pos.x, WINDOW_BOTTOM_Y + (size.y / 2.0), 0.0),
                    scale: size.extend(1.0),
                    ..default()
                },
                ..default()
            },
            body: RigidBody::Fixed,
            collider: Collider::cuboid(0.5, 0.5),
        }
    }
}
