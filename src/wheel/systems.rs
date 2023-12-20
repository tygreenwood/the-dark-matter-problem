use std::f32::consts::PI;

use bevy::{
    prelude::{
        default, AssetServer, Commands, GlobalTransform, Input, KeyCode, Query, Res, Transform,
        Vec3, With,
    },
    sprite::SpriteBundle,
    time::Time,
};
use rand::Rng;

use crate::{
    platforms::FLOOR_THICKNESS, player::components::Player, setup::configs::WINDOW_BOTTOM_Y,
};

use super::components::Wheel;

pub fn setup_wheel(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-500., WINDOW_BOTTOM_Y + 380. + FLOOR_THICKNESS, -2.),
                scale: Vec3::new(5., 5., 1.),
                ..default()
            },
            texture: asset_server.load("wheel.png"),
            ..default()
        },
        Wheel { spin: 0. },
    ));

    commands.spawn({
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-500., WINDOW_BOTTOM_Y + 255. + FLOOR_THICKNESS, -1.),
                scale: Vec3::new(5., 5., 1.),
                ..default()
            },
            texture: asset_server.load("WheelStand.png"),
            ..default()
        }
    });
}

pub fn spin_wheel(
    query_player: Query<&GlobalTransform, With<Player>>,
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
        println!("Spinning: {} degrees", rot.spin);
    }
}

pub fn spin(mut query_wheel: Query<(&mut Transform, &mut Wheel)>, time: Res<Time>) {
    if let Ok((mut wheel, mut rot)) = query_wheel.get_single_mut() {
        if rot.spin > 0. {
            wheel.rotate_z(300. * (PI / 180.) * time.delta_seconds());
            rot.spin -= 300. * time.delta_seconds();
            if rot.spin < 0. {
                rot.spin = 0.;
            }
        }
    }
}
