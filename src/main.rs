use bevy::{
    prelude::*,
    window::{WindowMode, WindowResolution},
};
use bevy_rapier2d::{prelude::*, render::RapierDebugRenderPlugin};

mod camera;
mod platforms;
mod player;
mod setup;

use camera::CameraPlugin;
use platforms::systems::setup_entities;
use player::PlayerPlugin;
use setup::{systems::setup_graphics, SetupPlugin, WINDOW_HEIGHT, WINDOW_WIDTH};

const COLOR_BACKGROUND: Color = Color::rgb(0.29, 0.31, 0.41);

fn main() {
    App::new()
        .insert_resource(ClearColor(COLOR_BACKGROUND))
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "The Dark Matter Problem".to_string(),
                    resizable: true,
                    resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT)
                        .with_scale_factor_override(1.0),
                    mode: WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            }),
        )
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins((SetupPlugin, PlayerPlugin, CameraPlugin))
        .add_systems(
            Startup,
            (setup_entities.after(setup_graphics), setup_graphics),
        )
        .run();
}
