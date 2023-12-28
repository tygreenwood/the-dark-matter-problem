use bevy::prelude::*;

pub const COLOR_PLATFORM: Color = Color::rgb(0.13, 0.13, 0.23);

pub const FLOOR_THICKNESS: f32 = 10.0;

pub const COLOR_FLOOR: Color = Color::rgb(0.45, 0.55, 0.66);

pub const TRIANGULAR_PLATFORM_TALL_PATH: &str = "sprites/platforms/triangular/PlatformTall.png";
pub const TRIANGULAR_PLATFORM_WIDE_PATH: &str = "sprites/platforms/triangular/PlatformWide.png";

pub const RECTANGULAR_PLATFORM_3X1_PATH: &str = "sprites/platforms/rectangular/Platform3x1.png";

pub const MODULAR_PLATFORM_CENTER_BLOCK_PATH: &str = "sprites/platforms/modular/CenterBlock.png";
pub const MODULAR_PLATFORM_EDGE_BLOCK_PATH: &str = "sprites/platforms/modular/EdgeBlock.png";

pub struct PlatformLocation {
    pub x: f32,
    pub y: f32,
}

// On the following line, set the number after PlatformLocation to the number of elements
pub const TRIANGULAR_PLATFORM_TALL_POSITIONS: [PlatformLocation; 2] = [
    PlatformLocation { x: -20.0, y: -50.0 },
    PlatformLocation { x: 100.0, y: -40.0 },
];
