use bevy::{
    input::gamepad::{GamepadConnection, GamepadEvent},
    prelude::*,
};
use bevy_rapier2d::prelude::*;

use crate::{
    animations::components::{AnimationIndices, AnimationTimer},
    saves::resources::PositionSaveInformation,
    setup::configs::{WINDOW_BOTTOM_Y, WINDOW_LEFT_X},
};

use super::{
    components::{Cat, ControlledPlayer, Dash, DashCooldown, Jump, MyGamepad},
    configs::{
        PLAYER_ACCELERATION_FROM_GRAVITY, PLAYER_DASH_VELOCITY, PLAYER_INITIAL_JUMP_VELOCITY,
        PLAYER_RUNNING_ANIMATION_PATH, PLAYER_VELOCITY_X,
    },
};

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let player_texture_handle = asset_server.load(PLAYER_RUNNING_ANIMATION_PATH);
    let player_texture_atlas = TextureAtlas::from_grid(
        player_texture_handle,
        Vec2::new(31.0, 46.0),
        6,
        1,
        None,
        None,
    );
    let player_texture_atlas_handle = texture_atlases.add(player_texture_atlas);
    let player_animation_indices = AnimationIndices { first: 0, last: 5 };

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: player_texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(WINDOW_LEFT_X + 100.0, WINDOW_BOTTOM_Y + 300.0, 0.0),
                scale: Vec3::new(4., 4., 1.),
                ..default()
            },
            ..default()
        },
        ControlledPlayer,
        RigidBody::KinematicPositionBased,
        Collider::cuboid(15.5, 23.0),
        KinematicCharacterController::default(),
        player_animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

pub fn gamepad_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    for ev in gamepad_evr.read() {
        match &ev {
            GamepadEvent::Connection(connection) => {
                match &connection.connection {
                    GamepadConnection::Connected(info) => {
                        println!(
                            "New gamepad connected with ID: {:?}, name: {}",
                            connection.gamepad.id, info.name
                        );

                        // if we don't have any gamepad yet, use this one
                        if my_gamepad.is_none() {
                            commands.insert_resource(MyGamepad(connection.gamepad));
                        }
                    }
                    GamepadConnection::Disconnected => {
                        println!(
                            "Lost gamepad connection with ID: {:?}",
                            connection.gamepad.id
                        );

                        // if it's the one we previously associated with the player,
                        // disassociate it:
                        if let Some(MyGamepad(old_id)) = my_gamepad.as_deref() {
                            if old_id.id == connection.gamepad.id {
                                commands.remove_resource::<MyGamepad>();
                            }
                        }
                    }
                }
            }
            // other events are irrelevant
            _ => {}
        }
    }
}

pub fn movement(
    input: Res<Input<KeyCode>>,
    axes: Res<Axis<GamepadAxis>>,
    my_gamepad: Option<Res<MyGamepad>>,
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut KinematicCharacterController,
        &GlobalTransform,
        Option<&mut Dash>,
    )>,
    mut query_player_flip: Query<&mut TextureAtlasSprite, (With<ControlledPlayer>, Without<Cat>)>,
    mut pos_save: ResMut<PositionSaveInformation>,
) {
    let joystick_move = my_gamepad.map_or(0., |gp| {
        let axis_lx = GamepadAxis {
            gamepad: gp.0,
            axis_type: GamepadAxisType::LeftStickX,
        };
        axes.get(axis_lx).unwrap_or(0.)
    });

    let (entity, mut player, player_pos, dash_option) = query.single_mut();

    let mut movement = 0.0;

    let mut player_flip = query_player_flip.single_mut();

    if right(&input) || joystick_move > 0.2 {
        movement += time.delta_seconds() * PLAYER_VELOCITY_X;
        if player_flip.flip_x {
            player_flip.flip_x = false;
        }
    }

    if left(&input) || joystick_move < -0.2 {
        movement += time.delta_seconds() * PLAYER_VELOCITY_X * -1.0;
        if !player_flip.flip_x {
            player_flip.flip_x = true;
        }
    }

    if let Some(mut dash) = dash_option {
        match player.translation {
            Some(vec) => {
                player.translation = Some(Vec2::new(dash.velocity * time.delta_seconds(), vec.y))
            }
            None => player.translation = Some(Vec2::new(dash.velocity * time.delta_seconds(), 0.0)),
        }
        dash.timer.tick(time.delta());
        if dash.timer.just_finished() {
            commands.entity(entity).remove::<Dash>();
            commands
                .entity(entity)
                .insert(DashCooldown(Timer::from_seconds(1., TimerMode::Once)));
        }
    } else {
        match player.translation {
            Some(vec) => player.translation = Some(Vec2::new(movement, vec.y)), // update if it already exists
            None => player.translation = Some(Vec2::new(movement, 0.0)),
        }
    }

    pos_save.x = player_pos.translation().x;
    pos_save.y = player_pos.translation().y;
}

pub fn dash_cooldown(
    mut commands: Commands,
    mut query_dash_cooldown: Query<(Entity, &mut DashCooldown), Without<Dash>>,
    time: Res<Time>,
) {
    if let Ok((entity, mut dash_cooldown)) = query_dash_cooldown.get_single_mut() {
        dash_cooldown.0.tick(time.delta());
        if dash_cooldown.0.just_finished() {
            commands.entity(entity).remove::<DashCooldown>();
        }
    }
}

pub fn dash(
    input: Res<Input<KeyCode>>,
    buttons: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut commands: Commands,
    query: Query<Entity, (Without<Dash>, Without<DashCooldown>, With<ControlledPlayer>)>,
) {
    if query.is_empty() {
        return;
    }

    let (dash_controller, joystick_move) = my_gamepad.map_or((false, 0.0), |gp| {
        let dash_button = GamepadButton {
            gamepad: gp.0,
            button_type: GamepadButtonType::East,
        };
        let axis_lx = GamepadAxis {
            gamepad: gp.0,
            axis_type: GamepadAxisType::LeftStickX,
        };
        (
            buttons.pressed(dash_button),
            axes.get(axis_lx).unwrap_or(0.),
        )
    });

    if !input.pressed(KeyCode::Q) && !dash_controller {
        return;
    }

    let entity = query.single();

    let mut dash = 0.0;
    if left(&input) || joystick_move < -0.2 {
        dash = -1. * PLAYER_DASH_VELOCITY;
    } else if right(&input) || joystick_move > 0.2 {
        dash = PLAYER_DASH_VELOCITY;
    }

    commands.entity(entity).insert(Dash {
        velocity: dash,
        timer: Timer::from_seconds(0.25, TimerMode::Once),
    });
}

pub fn jump(
    input: Res<Input<KeyCode>>,
    buttons: Res<Input<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut commands: Commands,
    query: Query<
        (Entity, &KinematicCharacterControllerOutput),
        (With<KinematicCharacterController>, Without<Jump>),
    >,
) {
    if query.is_empty() {
        return;
    }

    let up_controller = my_gamepad.map_or(false, |gp| {
        let jump_button = GamepadButton {
            gamepad: gp.0,
            button_type: GamepadButtonType::South,
        };
        buttons.pressed(jump_button)
    });

    let (player, output) = query.single();

    if (up(&input) || up_controller) && output.grounded {
        commands.entity(player).insert(Jump {
            time: 0.0,
            initial_velocity: PLAYER_INITIAL_JUMP_VELOCITY,
        });
    } else if output.grounded == false {
        commands.entity(player).insert(Jump {
            time: 0.0,
            initial_velocity: 0.0,
        });
    }
}

pub fn remove_jump(
    mut commands: Commands,
    query: Query<(Entity, &KinematicCharacterControllerOutput), With<Jump>>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, output) = query.single();

    if output.grounded {
        commands.entity(entity).remove::<Jump>();
    }
}

pub fn vertical_velocity(
    time: Res<Time>,
    mut query: Query<(&mut KinematicCharacterController, &mut Jump, &mut Transform)>,
) {
    if query.is_empty() {
        return;
    }

    let (mut player, mut jump, mut player_transform) = query.single_mut();

    let cur_time_dif = time.delta().as_secs_f32();

    let v_initial = jump.initial_velocity + PLAYER_ACCELERATION_FROM_GRAVITY * jump.time;

    jump.time += cur_time_dif;

    let v_final = jump.initial_velocity + PLAYER_ACCELERATION_FROM_GRAVITY * jump.time;

    let movement = (v_initial + v_final) / 2. * cur_time_dif;

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
        None => player.translation = Some(Vec2::new(0.0, movement)),
    }

    if player_transform.translation.y < -800. {
        player_transform.translation.x = 0.;
        player_transform.translation.y = -100.;
    }
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query_player: Query<
        (
            &AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
        ),
        (With<ControlledPlayer>, Without<Cat>),
    >,
    mut query_cat: Query<
        (
            &AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
        ),
        (With<Cat>, Without<ControlledPlayer>),
    >,
    input: Res<Input<KeyCode>>,
    axes: Res<Axis<GamepadAxis>>,
    my_gamepad: Option<Res<MyGamepad>>,
) {
    let joystick_move = if let Some(gp) = my_gamepad {
        let axis_lx = GamepadAxis {
            gamepad: gp.0,
            axis_type: GamepadAxisType::LeftStickX,
        };
        if let Some(x) = axes.get(axis_lx) {
            x
        } else {
            0.
        }
    } else {
        0.
    };

    for (indices, mut timer, mut sprite) in &mut query_player {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index != indices.last
                && (sprite.index != indices.first
                    || (left(&input) || right(&input))
                    || joystick_move.abs() > 0.2)
            {
                sprite.index + 1
            } else {
                indices.first
            };
        }
    }

    for (indices, mut timer, mut sprite) in &mut query_cat {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index != indices.last
                && (sprite.index != indices.first
                    || (left(&input) || right(&input))
                    || joystick_move.abs() > 0.2)
            {
                sprite.index + 1
            } else {
                indices.first
            };
        }
    }
}

fn left(input: &Res<Input<KeyCode>>) -> bool {
    input.pressed(KeyCode::Left) || input.pressed(KeyCode::A)
}

fn right(input: &Res<Input<KeyCode>>) -> bool {
    input.pressed(KeyCode::Right) || input.pressed(KeyCode::D)
}

fn up(input: &Res<Input<KeyCode>>) -> bool {
    input.pressed(KeyCode::Up) || input.pressed(KeyCode::W)
}

pub fn cleanup(
    mut commands: Commands,
    query_sprites: Query<Entity, Or<(With<ControlledPlayer>, With<Cat>)>>,
) {
    for entity in &query_sprites {
        commands.entity(entity).despawn();
    }
}
