use bevy::prelude::*;

use crate::setup::configs::WINDOW_BOTTOM_Y;

pub const FLOOR_THICKNESS: f32 = 100.0;

pub const TRIANGULAR_PLATFORM_TALL_PATH: &str = "sprites/platforms/triangular/PlatformTall.png";
pub const TRIANGULAR_PLATFORM_WIDE_PATH: &str = "sprites/platforms/triangular/PlatformWide.png";

pub const RECTANGULAR_PLATFORM_3X1_PATH: &str = "sprites/platforms/rectangular/Platform3x1.png";

pub const MODULAR_PLATFORM_CENTER_BLOCK_PATH: &str = "sprites/platforms/modular/CenterBlock.png";
pub const MODULAR_PLATFORM_EDGE_BLOCK_PATH: &str = "sprites/platforms/modular/EdgeBlock.png";

pub const LEVEL_ENVIRONMENT_PATH: &str = "sprites/platforms/level/environment.png";

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

pub fn modular_platform_center_vertices() -> Vec<Vec2> {
    let mut vertices = Vec::new();
    vertices.push(Vec2::new(-10., 10.));
    vertices.push(Vec2::new(10., 10.));
    vertices.push(Vec2::new(10., -10.));
    vertices.push(Vec2::new(-10., -10.));
    vertices
}

pub fn modular_platform_center_indices() -> Vec<[u32; 2]> {
    let mut indices = Vec::new();
    indices.push([0, 1]);
    indices.push([2, 3]);
    indices
}

pub fn modular_platform_edge_left_vertices() -> Vec<Vec2> {
    let mut vertices = Vec::new();
    vertices.push(Vec2::new(10., -10.));
    vertices.push(Vec2::new(-10., -10.));
    vertices.push(Vec2::new(-10., 10.));
    vertices.push(Vec2::new(10., 10.));
    vertices
}

pub fn modular_platform_edge_right_vertices() -> Vec<Vec2> {
    let mut vertices = Vec::new();
    vertices.push(Vec2::new(-10., -10.));
    vertices.push(Vec2::new(10., -10.));
    vertices.push(Vec2::new(10., 10.));
    vertices.push(Vec2::new(-10., 10.));
    vertices
}

pub fn environment_level_vertices() -> Vec<Vec2> {
    let mut vertices = Vec::new();

    //Bottom left island
    vertices.push(Vec2::new(-358., -203.));
    vertices.push(Vec2::new(-358., -163.));
    vertices.push(Vec2::new(-298., -163.));
    vertices.push(Vec2::new(-298., -143.));
    vertices.push(Vec2::new(-238., -143.));
    vertices.push(Vec2::new(-238., -165.));
    vertices.push(Vec2::new(-138., -165.));
    vertices.push(Vec2::new(-138., -178.));
    vertices.push(Vec2::new(-143., -203.));

    //First wide platform
    vertices.push(Vec2::new(-219., -110.));
    vertices.push(Vec2::new(-187., -110.));
    vertices.push(Vec2::new(-187., -115.));
    vertices.push(Vec2::new(-203., -124.));
    vertices.push(Vec2::new(-219., -115.));

    //Second wide platform
    vertices.push(Vec2::new(-133., -147.));
    vertices.push(Vec2::new(-101., -147.));
    vertices.push(Vec2::new(-101., -152.));
    vertices.push(Vec2::new(-117., -161.));
    vertices.push(Vec2::new(-133., -152.));

    vertices
}

pub fn environment_level_indices() -> Vec<[u32; 2]> {
    let mut indices = Vec::new();

    //Bottom left island
    indices.push([0, 1]);
    indices.push([1, 2]);
    indices.push([2, 3]);
    indices.push([3, 4]);
    indices.push([4, 5]);
    indices.push([5, 6]);
    indices.push([6, 7]);
    indices.push([7, 8]);

    //First wide platform
    indices.push([9, 10]);
    indices.push([10, 11]);
    indices.push([11, 12]);
    indices.push([12, 13]);
    indices.push([13, 9]);

    //Second wide platform
    indices.push([14, 15]);
    indices.push([15, 16]);
    indices.push([16, 17]);
    indices.push([17, 18]);
    indices.push([18, 14]);

    indices
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

pub const MODULAR_PLATFORM_POSITIONS: [ModularPlatformLocation; 3] = [
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
    ModularPlatformLocation {
        x: 0.0,
        y: WINDOW_BOTTOM_Y + 50.,
        center_count: 100,
    },
];
