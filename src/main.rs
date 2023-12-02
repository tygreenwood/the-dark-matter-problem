use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).add_systems(Startup, setup).add_systems(Update, player_move).run();
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((SpriteBundle {
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
    }, Player));
}

fn player_move(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Player>>, time: Res<Time>) {
    let mut player_transform = query.single_mut();
    let mut direction = 0.0;
    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    let new_player_transform = player_transform.translation.x + (direction * 100. * time.delta_seconds());

    player_transform.translation.x = new_player_transform;
}
