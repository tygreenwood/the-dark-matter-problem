use bevy::{prelude::*, window::PrimaryWindow};

use crate::player::components::MyGamepad;

use super::{
    configs::{AppStates, WINDOW_HEIGHT},
    resources::DisplayScale,
};

pub fn setup_graphics(
    mut scale: ResMut<DisplayScale>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    scale.0 = WINDOW_HEIGHT / window.resolution.height() as f32;
}

pub fn hide_cursor(mut query_primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = query_primary_window.single_mut();
    window.cursor.visible = false;
}

pub fn show_cursor(mut query_primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = query_primary_window.single_mut();
    window.cursor.visible = true;
}

pub fn esc_to_menu(
    input: Res<Input<KeyCode>>,
    buttons: Res<Input<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut app_state_next_state: ResMut<NextState<AppStates>>,
) {
    let menu_controller = my_gamepad.map_or(false, |gp| {
        let start_button = GamepadButton {
            gamepad: gp.0,
            button_type: GamepadButtonType::Start,
        };
        buttons.pressed(start_button)
    });

    if input.pressed(KeyCode::Escape) || menu_controller {
        app_state_next_state.set(AppStates::MainMenu);
    }
}
