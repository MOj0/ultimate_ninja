// pub const WIDTH: u32 = 1024;
// pub const HEIGHT: u32 = 768;
pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;
pub const GRID_SIZE: u32 = 40; // size 40 produces 20x15 grid (at 800x600)

/// Numer of characters in level text files
pub const LEVEL_SIZE: (u32, u32) = (30, 20);
pub const MAX_WORLD_X: u32 = GRID_SIZE * LEVEL_SIZE.0;
pub const MAX_WORLD_Y: u32 = GRID_SIZE * LEVEL_SIZE.1;

pub const GRAY_COLOR: ggez::graphics::Color = ggez::graphics::Color::new(0.5, 0.5, 0.5, 0.5);

pub const PI: f32 = std::f32::consts::PI;

pub const SPRITE_SIZE: u32 = 108;
pub const SPRITE_SCALE: glam::Vec2 = glam::vec2(0.33, 0.33);

pub const ENTITY_SIZE: f32 = 8.;

pub const PLAYER_SPEED: f32 = 3.;

pub const TARGET_SPEED: f32 = 1.;
pub const GUARD_SPEED: f32 = 1.8;
pub const GUARD_SPEED_FAST: f32 = 2.8;
pub const GUARD_FOV: f32 = PI / 6.;
pub const GUARD_VIEW_DISTANCE: f32 = 170.;

pub const ANIMATION_SPEED: f32 = 0.1575;

pub const DOUBLE_PRESS_TIME: f32 = 0.5; // sec
pub const HOLD_THRESHOLD_TIME: f32 = 0.4; // sec

pub const TELEPORT_COST_INTIAL: f32 = 25.;
pub const TELEPORT_COST: f32 = 50.;

pub const MENU_RECT_DIM: glam::Vec2 = glam::vec2(200., 75.);

pub const MENU_PLAY_POS: glam::Vec2 = glam::vec2(550., 250.);
pub const MENU_INFO_POS: glam::Vec2 = glam::vec2(550., 400.);
pub const MENU_BACK_POS: glam::Vec2 = glam::vec2(30., 30.);
pub const MENU_OK_POS: glam::Vec2 = glam::vec2(330., 500.);

pub const CAMERA_DEFAULT_LERP_DELTA: f32 = 0.33;

pub const LEVEL_ANIMATION_TIME: f32 = 4.;
