use bevy::{prelude::*, window::PrimaryWindow};

use super::{configs::WINDOW_HEIGHT, resources::DisplayScale};

pub fn setup_graphics(
    mut scale: ResMut<DisplayScale>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    scale.0 = WINDOW_HEIGHT / window.resolution.height() as f32;
}
