use crate::constants;
use crate::entities::exit::Exit;
use crate::entities::guard::Guard;
use crate::entities::player::Player;
use crate::entities::target::Target;
use crate::entities::wall::Wall;
use crate::SpriteComponent;
use std::io::Read;

use crate::Game;

const ALL_LEVELS: &[&str] = &[
    "levels/level1.txt",
    "levels/level2.txt",
    "levels/level3.txt",
    "levels/level4.txt",
    "levels/level5.txt",
    "levels/level6.txt",
    "levels/level7.txt",
    "levels/level8.txt",
];

pub const LEVEL_COUNT: usize = ALL_LEVELS.len();

pub fn load_level(
    ctx: &mut ggez::Context,
    quad_ctx: &mut ggez::miniquad::GraphicsContext,
    game_state: &mut Game,
    level_index: usize,
    is_proceed: bool,
) {
    assert!(
        level_index < LEVEL_COUNT,
        "level index: {} out of bounds for length: {}",
        level_index,
        LEVEL_COUNT
    );

    game_state.reset_state(is_proceed);

    let assets = &game_state.assets;

    let level_filename = ALL_LEVELS[level_index];
    let mut file = ggez::filesystem::open(ctx, level_filename).expect("could not open level file");
    let mut level = String::new();
    file.read_to_string(&mut level).unwrap();

    let (mut x, mut y): (i32, i32) = (0, 0);
    for char in level.chars() {
        let position = glam::vec2(
            (x as u32 * constants::GRID_SIZE) as f32,
            (y as u32 * constants::GRID_SIZE) as f32,
        );
        let position_center = position + glam::Vec2::splat(constants::GRID_SIZE as f32 / 2.);

        match char {
            'p' => {
                game_state.player = Player::new(
                    ctx,
                    quad_ctx,
                    position_center,
                    &assets,
                    ggez::graphics::Color::BLACK,
                );

                game_state
                    .camera
                    .update(game_state.player.transform.clone());
            }
            't' => {
                game_state.target =
                    Target::new(position_center, &assets, ggez::graphics::Color::GREEN)
            }
            'g' => game_state.guards.push(Guard::new(
                ctx,
                quad_ctx,
                position_center,
                &assets,
                ggez::graphics::Color::RED,
            )),
            'x' => game_state.walls.push(Wall::new(
                position,
                constants::GRID_SIZE as f32,
                constants::GRID_SIZE as f32,
                SpriteComponent::new(assets.wall.clone(), ggez::graphics::Color::WHITE),
            )),
            'b' => game_state.walls.push(Wall::new(
                position,
                constants::GRID_SIZE as f32,
                constants::GRID_SIZE as f32,
                SpriteComponent::new(assets.box1.clone(), ggez::graphics::Color::WHITE),
            )),
            'B' => game_state.walls.push(Wall::new(
                position,
                constants::GRID_SIZE as f32,
                constants::GRID_SIZE as f32,
                SpriteComponent::new(assets.box2.clone(), ggez::graphics::Color::WHITE),
            )),
            'e' => {
                game_state.exit = Exit::new(
                    position_center,
                    SpriteComponent::new(assets.exit.clone(), ggez::graphics::Color::WHITE),
                )
            }
            '\n' => {
                x = -1;
                y += 1;
            }
            _ => (),
        }
        x += 1;
    }
}
