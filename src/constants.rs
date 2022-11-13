// pub const WIDTH: u32 = 1024;
// pub const HEIGHT: u32 = 768;
pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;
pub const GRID_SIZE: u32 = 40; // size 40 produces 20x15 grid (at 800x600)

pub const PI: f32 = std::f32::consts::PI;

pub const SPRITE_SIZE: u32 = 108;
pub const SPRITE_SCALE: glam::Vec2 = glam::vec2(0.33, 0.33);

pub const ENTITY_SIZE: f32 = 8.;

pub const PLAYER_SPEED: f32 = 3.;

pub const TARGET_SPEED: f32 = 2.;
pub const GUARD_SPEED: f32 = 2.2;
pub const GUARD_FOV: f32 = PI / 6.;
pub const GUARD_VIEW_DISTANCE: f32 = 200.;

pub const ANIMATION_SPEED: f32 = 0.1575;

pub const DOUBLE_PRESS_TIME: f32 = 0.5; // sec

pub const TELEPORT_COST_INTIAL: f32 = 25.;
pub const TELEPORT_COST: f32 = 50.;
