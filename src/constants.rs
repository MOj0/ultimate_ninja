pub const TARGET_FPS: f32 = 60.;

pub const CONFIG_FILENAME: &str = "ultimate_ninja_settings.json";

// pub const WIDTH: u32 = 1920;
// pub const HEIGHT: u32 = 1080;
// pub const WIDTH: u32 = 1024;
// pub const HEIGHT: u32 = 768;
pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

pub const LEVEL_BLOCK_SIZE: u32 = 40; // size 40 produces 20x15 grid (at 800x600)

pub const GRID_CELL_SIZE: usize = 5 * LEVEL_BLOCK_SIZE as usize;

/// Number of characters in level text files
pub const LEVEL_SIZE: (u32, u32) = (30, 20);
pub const MAX_WORLD_X: u32 = LEVEL_BLOCK_SIZE * LEVEL_SIZE.0;
pub const MAX_WORLD_Y: u32 = LEVEL_BLOCK_SIZE * LEVEL_SIZE.1;

pub const BG_COLOR: ggez::graphics::Color = ggez::graphics::Color::new(0.125, 0.125, 0.125, 0.5);

pub const GLOBAL_BRIGHTNESS: f32 = 0.2;

pub const PI: f32 = std::f32::consts::PI;

pub const SPRITE_SIZE: u32 = 108;
pub const SPRITE_SCALE: glam::Vec2 = glam::vec2(0.33, 0.33);

pub const ENTITY_SIZE: f32 = 8.;

pub const PLAYER_ATTACK_RANGE: f32 = 42.;
pub const PLAYER_SPEED_SLOW: f32 = 1.;
pub const PLAYER_SPEED: f32 = 3.;
pub const PLAYER_SPEED_FAST: f32 = 4.5;
pub const SOUND_RADIUS_SCALE: f32 = 0.5;

pub const TARGET_SPEED: f32 = 1.;
pub const GUARD_SPEED_SLOW: f32 = 0.8;
pub const GUARD_SPEED: f32 = 1.4;
pub const GUARD_SPEED_MEDIUM: f32 = 2.5;
pub const GUARD_SPEED_FAST: f32 = 3.2;
pub const GUARD_FOV: f32 = PI / 6.;
pub const GUARD_FOV_SMALL: f32 = PI / 18.;
pub const GUARD_VIEW_DISTANCE: f32 = 170.;
pub const GUARD_VIEW_DISTANCE_MEDIUM: f32 = 250.;
pub const GUARD_VIEW_DISTANCE_LONG: f32 = 500.;

pub const ANIMATION_SPEED: f32 = 0.1575;

pub const DOUBLE_PRESS_TIME: f32 = 0.5; // sec
pub const HOLD_THRESHOLD_TIME: f32 = 0.4; // sec

pub const TELEPORT_COST_INTIAL: f32 = 25.;
pub const TELEPORT_COST: f32 = 50.;

pub const N_FOV_RAYS: u32 = 9;

pub const BTN_DIM_RECT: glam::Vec2 = glam::vec2(WIDTH as f32 * 0.3, HEIGHT as f32 * 0.125);
pub const BTN_DIM_SQUARE: glam::Vec2 = glam::vec2(WIDTH as f32 * 0.0625, HEIGHT as f32 * 0.083);

pub const BTN_PLAY_POS: glam::Vec2 = glam::vec2(WIDTH as f32 * 0.5825, HEIGHT as f32 * 0.33);
pub const BTN_INFO_POS: glam::Vec2 = glam::vec2(WIDTH as f32 * 0.5825, HEIGHT as f32 * 0.5833);
pub const BTN_BACK_POS: glam::Vec2 = glam::vec2(WIDTH as f32 * 0.0375, HEIGHT as f32 * 0.18);
pub const BTN_BOTTOM_LEFT_POS: glam::Vec2 = glam::vec2(WIDTH as f32 * 0.125, HEIGHT as f32 * 0.833);
pub const BTN_BOTTOM_RIGHT_POS: glam::Vec2 =
    glam::vec2(WIDTH as f32 * 0.5825, HEIGHT as f32 * 0.833);

pub const BTN_BOTTOM_LEFT_POS1: glam::Vec2 = glam::vec2(WIDTH as f32 * 0.025, HEIGHT as f32 * 0.59);
pub const BTN_BOTTOM_LEFT_POS2: glam::Vec2 =
    glam::vec2(WIDTH as f32 * 0.025, HEIGHT as f32 * 0.7166);
pub const BTN_BOTTOM_LEFT_POS3: glam::Vec2 =
    glam::vec2(WIDTH as f32 * 0.025, HEIGHT as f32 * 0.833);

pub const CAMERA_DEFAULT_LERP_DELTA: f32 = 0.33;

pub const LEVEL_ANIMATION_TIME: f32 = 4.;
