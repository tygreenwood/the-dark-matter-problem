use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Platform;

#[derive(Bundle)]
pub struct PlatformBundle {
    sprite_bundle: SpriteBundle,
    body: RigidBody,
    collider: Collider,
    platform: Platform,
}

impl PlatformBundle {
    pub fn new(
        pos: Vec2,
        size: Vec2,
        asset: Handle<Image>,
        scale_platform: Option<Vec2>,
        flip_sprite: bool,
        collider_option: Option<Collider>,
    ) -> Self {
        let scale = if let Some(scale_result) = scale_platform {
            scale_result.extend(1.)
        } else {
            Vec3::new(5., 5., 1.)
        };

        let collider = if let Some(collider_result) = collider_option {
            collider_result
        } else {
            Collider::cuboid(size.x, size.y)
        };

        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    flip_x: flip_sprite,
                    ..default()
                },
                texture: asset,
                transform: Transform {
                    translation: Vec3::new(pos.x, pos.y, 0.0),
                    scale,
                    ..default()
                },
                ..default()
            },
            body: RigidBody::Fixed,
            collider,
            platform: Platform,
        }
    }
}
