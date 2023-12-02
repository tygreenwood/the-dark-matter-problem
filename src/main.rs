use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, *};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_systems(Startup, setup)
        .add_systems(Update, player_move)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct PlatformBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
    rigid_body: RigidBody,
}

impl PlatformBundle {
    fn new(location: Vec2, size: Vec2) -> PlatformBundle {
        PlatformBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.extend(0.0),
                    scale: size.extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(1., 1., 1.),
                    ..default()
                },
                ..default()
            },
            collider: Collider::cuboid(size.x / 2., size.y / 2.),
            rigid_body: RigidBody::Fixed,
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0., 0., 0.),
                    scale: Vec3::new(20., 20., 0.),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(1., 0., 0.),
                    ..default()
                },
                ..default()
            },
            Player,
            RigidBody::Dynamic,
            Collider::cuboid(10., 10.),
        ))
        .insert(LockedAxes::ROTATION_LOCKED);

    commands.spawn(PlatformBundle::new(
        Vec2::new(0., -100.),
        Vec2::new(100., 100.),
    ));
}

fn player_move(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut player_transform = query.single_mut();
    let mut direction = 0.0;
    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    let new_player_transform =
        player_transform.translation.x + (direction * 100. * time.delta_seconds());

    player_transform.translation.x = new_player_transform;
}
