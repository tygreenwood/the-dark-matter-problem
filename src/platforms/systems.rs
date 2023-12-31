use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::setup::configs::{WINDOW_BOTTOM_Y, WINDOW_WIDTH};

use super::{
    components::{Platform, PlatformBundle},
    configs::{
        triangular_platform_tall_vertices, triangular_platform_wide_vertices, COLOR_FLOOR,
        FLOOR_THICKNESS, MODULAR_PLATFORM_CENTER_BLOCK_PATH, MODULAR_PLATFORM_EDGE_BLOCK_PATH,
        MODULAR_PLATFORM_POSITIONS, RECTANGULAR_PLATFORM_3X1_PATH,
        RECTANGULAR_PLATFORM_3X1_POSITIONS, TRIANGULAR_PLATFORM_TALL_PATH,
        TRIANGULAR_PLATFORM_TALL_POSITIONS, TRIANGULAR_PLATFORM_WIDE_PATH,
        TRIANGULAR_PLATFORM_WIDE_POSITIONS,
    },
};

pub fn setup_entities(mut commands: Commands, asset_server: Res<AssetServer>) {
    for loc in TRIANGULAR_PLATFORM_TALL_POSITIONS {
        commands.spawn(PlatformBundle::new(
            Vec2::new(loc.x, loc.y),
            Vec2::new(12., 14.),
            asset_server.load(TRIANGULAR_PLATFORM_TALL_PATH),
            None,
            false,
            Some(Collider::polyline(
                triangular_platform_tall_vertices(),
                None,
            )),
        ));
    }

    for loc in TRIANGULAR_PLATFORM_WIDE_POSITIONS {
        commands.spawn(PlatformBundle::new(
            Vec2::new(loc.x, loc.y),
            Vec2::new(16., 7.),
            asset_server.load(TRIANGULAR_PLATFORM_WIDE_PATH),
            None,
            false,
            Some(Collider::polyline(
                triangular_platform_wide_vertices(),
                None,
            )),
        ));
    }

    for loc in RECTANGULAR_PLATFORM_3X1_POSITIONS {
        commands.spawn(PlatformBundle::new(
            Vec2::new(loc.x, loc.y),
            Vec2::new(200., 50.5),
            asset_server.load(RECTANGULAR_PLATFORM_3X1_PATH),
            Some(Vec2::new(1., 1.)),
            false,
            None,
        ));
    }

    for loc in MODULAR_PLATFORM_POSITIONS {
        commands.spawn(PlatformBundle::new(
            Vec2::new(
                loc.x + (-100. * (loc.center_count as f32 / 2.)) - 50.,
                loc.y,
            ),
            Vec2::new(10., 10.),
            asset_server.load(MODULAR_PLATFORM_EDGE_BLOCK_PATH),
            None,
            true,
            None,
        ));
        for i in 1..=loc.center_count {
            commands.spawn(PlatformBundle::new(
                Vec2::new(
                    loc.x + ((i as f32 - (loc.center_count as f32 / 2.)) * 100. - 50.),
                    loc.y,
                ),
                Vec2::new(10., 10.),
                asset_server.load(MODULAR_PLATFORM_CENTER_BLOCK_PATH),
                None,
                false,
                None,
            ));
        }
        commands.spawn(PlatformBundle::new(
            Vec2::new(loc.x + (100. * (loc.center_count as f32 / 2.)) + 50., loc.y),
            Vec2::new(10., 10.),
            asset_server.load(MODULAR_PLATFORM_EDGE_BLOCK_PATH),
            None,
            false,
            None,
        ));
    }

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
