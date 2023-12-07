use bevy::prelude::Resource;

use super::{WINDOW_HEIGHT, WINDOW_WIDTH};

#[derive(Resource)]
pub struct DisplayResolution {
    pub x: f32,
    pub y: f32,
}

impl Default for DisplayResolution {
    fn default() -> Self {
        DisplayResolution {
            x: WINDOW_WIDTH,
            y: WINDOW_HEIGHT,
        }
    }
}

#[derive(Resource)]
pub struct DisplayScale(pub f32);

impl Default for DisplayScale {
    fn default() -> Self {
        DisplayScale(1.0)
    }
}
