use bevy::prelude::*;

use crate::setup::configs::AppStates;

use self::systems::*;

mod components;
mod configs;
mod systems;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppStates::MainMenu), spawn_main_menu)
            .add_systems(
                Update,
                (
                    interact_with_play_button,
                    interact_with_load_button,
                    interact_with_quit_button,
                )
                    .run_if(in_state(AppStates::MainMenu)),
            )
            .add_systems(OnExit(AppStates::MainMenu), cleanup);
    }
}
