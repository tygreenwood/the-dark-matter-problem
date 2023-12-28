use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

use crate::{
    platforms::configs::FLOOR_THICKNESS, player::components::ControlledPlayer,
    saves::resources::WheelSaveInformation, setup::configs::WINDOW_BOTTOM_Y,
};

use super::{
    components::{Wheel, WheelStand},
    configs::{WHEEL_PATH, WHEEL_STAND_PATH},
};

pub fn setup_wheel(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-500., WINDOW_BOTTOM_Y + 380. + FLOOR_THICKNESS, -2.),
                scale: Vec3::new(5., 5., 1.),
                ..default()
            },
            texture: asset_server.load(WHEEL_PATH),
            ..default()
        },
        Wheel { spin: 0. },
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-500., WINDOW_BOTTOM_Y + 255. + FLOOR_THICKNESS, -1.),
                scale: Vec3::new(5., 5., 1.),
                ..default()
            },
            texture: asset_server.load(WHEEL_STAND_PATH),
            ..default()
        },
        WheelStand,
    ));
}

pub fn spin_wheel(
    query_player: Query<&GlobalTransform, With<ControlledPlayer>>,
    mut query_wheel: Query<(&GlobalTransform, &mut Wheel)>,
    input: Res<Input<KeyCode>>,
) {
    let mut rng = rand::thread_rng();
    let player = query_player.single();
    let (wheel, mut rot) = query_wheel.single_mut();

    if (player.translation().x - wheel.translation().x).powi(2)
        + (player.translation().y - wheel.translation().y).powi(2)
        < 40000.
        && input.pressed(KeyCode::E)
        && rot.spin == 0.
    {
        rot.spin = rng.gen_range(0.0..360.) + 1080.;
    }
}

pub fn spin(
    mut query_wheel: Query<(&mut Transform, &mut Wheel, &GlobalTransform)>,
    time: Res<Time>,
    mut wheel_save: ResMut<WheelSaveInformation>,
) {
    if let Ok((mut wheel, mut rot, transform)) = query_wheel.get_single_mut() {
        if rot.spin > 0. {
            wheel.rotate_z(300. * (PI / 180.) * time.delta_seconds());
            rot.spin -= 300. * time.delta_seconds();
            if rot.spin < 0. {
                rot.spin = 0.;
            }
            wheel_save.rot = transform.to_scale_rotation_translation().1;
        }
    }
}

pub fn cleanup(
    mut commands: Commands,
    query_wheel_parts: Query<Entity, Or<(With<Wheel>, With<WheelStand>)>>,
) {
    for entity in &query_wheel_parts {
        commands.entity(entity).despawn();
    }
}
