use bevy::prelude::*;

pub const FLOOR_THICKNESS: f32 = 10.0;

pub const COLOR_FLOOR: Color = Color::rgb(0.45, 0.55, 0.66);

pub const TRIANGULAR_PLATFORM_TALL_PATH: &str = "sprites/platforms/triangular/PlatformTall.png";
pub const TRIANGULAR_PLATFORM_WIDE_PATH: &str = "sprites/platforms/triangular/PlatformWide.png";

pub const RECTANGULAR_PLATFORM_3X1_PATH: &str = "sprites/platforms/rectangular/Platform3x1.png";

pub const MODULAR_PLATFORM_CENTER_BLOCK_PATH: &str = "sprites/platforms/modular/CenterBlock.png";
pub const MODULAR_PLATFORM_EDGE_BLOCK_PATH: &str = "sprites/platforms/modular/EdgeBlock.png";

pub fn triangular_platform_tall_vertices() -> Vec<Vec2> {
    let mut vertices = Vec::new();
    vertices.push(Vec2::new(-12., 14.));
    vertices.push(Vec2::new(12., 14.));
    vertices.push(Vec2::new(12., 9.));
    vertices.push(Vec2::new(0., -14.));
    vertices.push(Vec2::new(-12., 9.));
    vertices.push(Vec2::new(-12., 14.));
    vertices
}

pub fn triangular_platform_wide_vertices() -> Vec<Vec2> {
    let mut vertices = Vec::new();
    vertices.push(Vec2::new(-16., 7.));
    vertices.push(Vec2::new(16., 7.));
    vertices.push(Vec2::new(16., 2.));
    vertices.push(Vec2::new(0., -7.));
    vertices.push(Vec2::new(-16., 2.));
    vertices.push(Vec2::new(-16., 7.));
    vertices
}

pub struct PlatformLocation {
    pub x: f32,
    pub y: f32,
}

pub struct ModularPlatformLocation {
    pub x: f32,
    pub y: f32,
    pub center_count: usize,
}

pub const TRIANGULAR_PLATFORM_TALL_POSITIONS: [PlatformLocation; 2] = [
    PlatformLocation { x: -20.0, y: -50.0 },
    PlatformLocation { x: 100.0, y: -40.0 },
];
pub const TRIANGULAR_PLATFORM_WIDE_POSITIONS: [PlatformLocation; 2] = [
    PlatformLocation {
        x: -300.0,
        y: -50.0,
    },
    PlatformLocation { x: 400.0, y: -40.0 },
];

pub const RECTANGULAR_PLATFORM_3X1_POSITIONS: [PlatformLocation; 1] = [PlatformLocation {
    x: -1200.0,
    y: -400.0,
}];

pub const MODULAR_PLATFORM_POSITIONS: [ModularPlatformLocation; 2] = [
    ModularPlatformLocation {
        x: -300.0,
        y: -200.0,
        center_count: 2,
    },
    ModularPlatformLocation {
        x: 500.0,
        y: -300.0,
        center_count: 3,
    },
];
