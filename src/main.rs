use bevy::{
    prelude::*,
    window::{WindowMode, WindowResolution},
};
use bevy_rapier2d::{prelude::*, render::RapierDebugRenderPlugin};

mod background;
mod camera;
mod main_menu;
mod mushrooms;
mod platforms;
mod player;
mod saves;
mod setup;
mod wheel;

use background::BackgroundPlugin;
use camera::CameraPlugin;
use main_menu::MainMenuPlugin;
use mushrooms::MushroomsPlugin;
use platforms::PlatformsPlugin;
use player::PlayerPlugin;
use saves::SavesPlugin;
use setup::{
    configs::{AppStates, WINDOW_HEIGHT, WINDOW_WIDTH},
    SetupPlugin,
};
use wheel::WheelPlugin;

const COLOR_BACKGROUND: Color = Color::rgb(0.29, 0.31, 0.41);

fn main() {
    App::new()
        .add_state::<AppStates>()
        .insert_resource(ClearColor(COLOR_BACKGROUND))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "The Dark Matter Problem".to_string(),
                        resizable: true,
                        resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                        mode: WindowMode::BorderlessFullscreen,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins((
            SavesPlugin,
            SetupPlugin,
            PlayerPlugin,
            CameraPlugin,
            PlatformsPlugin,
            BackgroundPlugin,
            MushroomsPlugin,
            WheelPlugin,
            MainMenuPlugin,
        ))
        .run();
}
