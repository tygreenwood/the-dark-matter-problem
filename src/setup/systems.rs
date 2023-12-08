use bevy::{prelude::*, window::PrimaryWindow};

pub fn setup_graphics(window_query: Query<&Window, With<PrimaryWindow>>) {
    {
        let win = window_query.single();
        println!("{}", win.resolution.scale_factor());
    }
}
