use crate::constants;
use crate::entities::exit::Exit;
use crate::entities::guard::Guard;
use crate::entities::player::Player;
use crate::entities::target::Target;
use crate::entities::wall::Wall;
use crate::SpriteComponent;
use std::io::Read;

use crate::GameState;

const ALL_LEVELS: &[&str] = &[
    "levels/level0.txt",
    "levels/level1.txt",
    "levels/level_end.txt",
];

pub const LEVEL_COUNT: usize = ALL_LEVELS.len();

pub fn load_level(
    ctx: &mut ggez::Context,
    quad_ctx: &mut ggez::miniquad::GraphicsContext,
    game_state: &mut GameState,
    level_index: usize,
) {
    assert!(
        level_index < LEVEL_COUNT,
        "level index: {} out of bounds for length: {}",
        level_index,
        LEVEL_COUNT
    );

    game_state.next_level();

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

        match char {
            'p' => {
                game_state.player = Player::new(
                    ctx,
                    quad_ctx,
                    position,
                    &assets,
                    ggez::graphics::Color::BLACK,
                )
            }
            't' => game_state.target = Target::new(position, &assets, ggez::graphics::Color::GREEN),
            'g' => game_state.guards.push(Guard::new(
                ctx,
                quad_ctx,
                position,
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
                    position,
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
