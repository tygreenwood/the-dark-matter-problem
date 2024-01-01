use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::animations::components::{AnimationIndices, AnimationTimer};

use super::{
    components::{Platform, PlatformBundle},
    configs::{
        environment_level_indices, environment_level_vertices, modular_platform_center_indices,
        modular_platform_center_vertices, modular_platform_edge_left_vertices,
        modular_platform_edge_right_vertices, triangular_platform_tall_vertices,
        triangular_platform_wide_vertices, LEVEL_ENVIRONMENT_PATH,
        MODULAR_PLATFORM_CENTER_BLOCK_PATH, MODULAR_PLATFORM_EDGE_BLOCK_PATH,
        MODULAR_PLATFORM_POSITIONS, RECTANGULAR_PLATFORM_3X1_PATH,
        RECTANGULAR_PLATFORM_3X1_POSITIONS, TRIANGULAR_PLATFORM_TALL_PATH,
        TRIANGULAR_PLATFORM_TALL_POSITIONS, TRIANGULAR_PLATFORM_WIDE_PATH,
        TRIANGULAR_PLATFORM_WIDE_POSITIONS,
    },
};

pub fn setup_entities(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
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
            Some(Collider::polyline(
                modular_platform_edge_left_vertices(),
                None,
            )),
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
                Some(Collider::polyline(
                    modular_platform_center_vertices(),
                    Some(modular_platform_center_indices()),
                )),
            ));
        }
        commands.spawn(PlatformBundle::new(
            Vec2::new(loc.x + (100. * (loc.center_count as f32 / 2.)) + 50., loc.y),
            Vec2::new(10., 10.),
            asset_server.load(MODULAR_PLATFORM_EDGE_BLOCK_PATH),
            None,
            false,
            Some(Collider::polyline(
                modular_platform_edge_right_vertices(),
                None,
            )),
        ));
    }

    let texture_handle = asset_server.load(LEVEL_ENVIRONMENT_PATH);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(716.0, 406.0), 12, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 0, last: 30 };

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(7000., 475., 0.),
                scale: Vec3::new(5., 5., 1.),
                ..default()
            },
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Platform,
        Collider::polyline(
            environment_level_vertices(),
            Some(environment_level_indices()),
        ),
    ));
}

pub fn cleanup(mut commands: Commands, query_platforms: Query<Entity, With<Platform>>) {
    for entity in &query_platforms {
        commands.entity(entity).despawn();
    }
}
