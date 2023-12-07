use bevy::{prelude::*, window::PrimaryWindow, winit::WinitWindows};

use crate::setup::WINDOW_HEIGHT;

use super::resources::{DisplayResolution, DisplayScale};

pub fn setup_graphics(
    mut resolution: ResMut<DisplayResolution>,
    mut scale: ResMut<DisplayScale>,
    winit_windows: NonSend<WinitWindows>,
    window_query: Query<Entity, With<PrimaryWindow>>,
) {
    if let Some(monitor) = window_query
        .get_single()
        .ok()
        .and_then(|entity| winit_windows.get_window(entity))
        .and_then(|winit_window| winit_window.current_monitor())
    {
        resolution.x = monitor.size().width as f32;
        resolution.y = monitor.size().height as f32;
        scale.0 = resolution.y / WINDOW_HEIGHT;
    }
}
