use bevy::prelude::*;

#[derive(Resource)]
pub struct DisplayScale(pub f32);

impl Default for DisplayScale {
    fn default() -> Self {
        DisplayScale(1.0)
    }
}
