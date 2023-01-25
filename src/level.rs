use crate::constants;
use crate::entities::exit::Exit;
use crate::entities::guards::guard_basic::GuardBasic;
use crate::entities::guards::guard_heavy::GuardHeavy;
use crate::entities::guards::guard_scout::GuardScout;
use crate::entities::player::Player;
use crate::entities::target::Target;
use crate::entities::wall::Wall;
use crate::tile_component::TileComponent;
use crate::util;
use crate::SpriteComponent;
use std::io::Read;

use crate::Game;

const ALL_LEVELS: &[&str] = &[
    "levels/tutorial1.txt",
    "levels/tutorial2.txt",
    "levels/tutorial3.txt",
    "levels/tutorial4.txt",
    "levels/tutorial5.txt",
    "levels/level1.txt",
    "levels/level2.txt",
    "levels/level3.txt",
    "levels/level4.txt",
    "levels/level5.txt",
    "levels/level6.txt",
    "levels/level7.txt",
    "levels/level8.txt",
    "levels/level9.txt",
    "levels/level10.txt",
    "levels/level11.txt",
    "levels/level12.txt",
];

pub const TUTORIAL_COUNT: usize = 5;
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
    let mut floor_tile = '-';

    for char in level.chars() {
        let position = glam::vec2(
            (x as u32 * constants::LEVEL_BLOCK_SIZE) as f32,
            (y as u32 * constants::LEVEL_BLOCK_SIZE) as f32,
        );
        let position_center = position + glam::Vec2::splat(constants::LEVEL_BLOCK_SIZE as f32 / 2.);

        if char != 'p' && char != 't' && char != 'g' && char != 's' && char != 'h' && char != 'e' {
            floor_tile = char;
        }

        match char {
            'p' => {
                game_state.player = Player::new(
                    ctx,
                    quad_ctx,
                    position_center,
                    &assets,
                    ggez::graphics::Color::BLACK,
                );
            }
            't' => {
                game_state.target =
                    Target::new(position_center, &assets, ggez::graphics::Color::GREEN)
            }
            'g' => game_state.guards_basic.push(GuardBasic::new(
                ctx,
                quad_ctx,
                position_center,
                &assets,
                ggez::graphics::Color::YELLOW,
                level_index < TUTORIAL_COUNT,
            )),
            's' => game_state.guards_scout.push(GuardScout::new(
                ctx,
                quad_ctx,
                position_center,
                &assets,
                ggez::graphics::Color::CYAN,
                level_index < TUTORIAL_COUNT,
            )),
            'h' => game_state.guards_heavy.push(GuardHeavy::new(
                ctx,
                quad_ctx,
                position_center,
                &assets,
                ggez::graphics::Color::RED,
                level_index < TUTORIAL_COUNT,
            )),
            'x' => game_state.walls.push(Wall::new(
                position,
                constants::LEVEL_BLOCK_SIZE as f32,
                constants::LEVEL_BLOCK_SIZE as f32,
                SpriteComponent::new(assets.wall.clone(), ggez::graphics::Color::WHITE),
            )),
            'b' => game_state.walls.push(Wall::new(
                position,
                constants::LEVEL_BLOCK_SIZE as f32,
                constants::LEVEL_BLOCK_SIZE as f32,
                SpriteComponent::new(assets.box1.clone(), ggez::graphics::Color::WHITE),
            )),
            'B' => game_state.walls.push(Wall::new(
                position,
                constants::LEVEL_BLOCK_SIZE as f32,
                constants::LEVEL_BLOCK_SIZE as f32,
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

        match floor_tile {
            '1' => game_state.floor_tiles.push(TileComponent::new(
                position,
                constants::LEVEL_BLOCK_SIZE as f32,
                constants::LEVEL_BLOCK_SIZE as f32,
                SpriteComponent::new(assets.floor1.clone(), ggez::graphics::Color::WHITE),
            )),
            '2' => game_state.floor_tiles.push(TileComponent::new(
                position,
                constants::LEVEL_BLOCK_SIZE as f32,
                constants::LEVEL_BLOCK_SIZE as f32,
                SpriteComponent::new(assets.floor2.clone(), ggez::graphics::Color::WHITE),
            )),
            _ => (),
        }

        x += 1;
    }
}

pub fn system(game_state: &mut Game) {
    match game_state.level_idx {
        0 => {
            let overlay_active = !game_state.exit.player_exited;
            game_state.overlay_system.set_active_at(1, overlay_active);
            game_state.overlay_system.set_active_at(2, overlay_active);

            let mut pos = game_state.player.transform.position;
            let mut text = "This is you";
            if game_state.target.is_dead() {
                pos = game_state.exit.transform.position;
                text = "Go through the exit to get to the next level"
            } else if (game_state.player.transform.position - glam::vec2(580., 420.)).length() > 50.
            {
                pos = game_state.target.transform.position;
                text = "This is your target\nEliminate the target to pass the level"
            }

            game_state.overlay_system.set_pos_at(
                1,
                pos + glam::vec2(-constants::ENTITY_SIZE * 4., constants::ENTITY_SIZE * 4.),
            );
            game_state.overlay_system.set_rot_at(1, -constants::PI / 4.);

            game_state
                .overlay_system
                .set_pos_at(2, pos + glam::vec2(0., constants::ENTITY_SIZE * 3.));

            game_state.overlay_system.set_text_at(2, text);
        }
        1 => {
            let overlay_active =
                !game_state.target.is_dead() || game_state.player.teleport.location.is_some();
            game_state.overlay_system.set_active_at(1, overlay_active);
            game_state.overlay_system.set_active_at(2, overlay_active);

            let mut pos =
                game_state.exit.transform.position + glam::vec2(0., constants::ENTITY_SIZE * 5.);
            let mut text = "You also have some special abilities which drain your stamina\nOne of them is teleport\nTry double-pressing the F key somewhere here to place the marker";
            if game_state.target.is_dead() {
                pos = game_state.player.transform.position;
                text = "To teleport back\ndouble-press the F key"
            } else if game_state.player.teleport.location.is_some() {
                pos = glam::vec2(
                    constants::MAX_WORLD_X as f32 / 2.,
                    constants::MAX_WORLD_Y as f32 / 2.,
                );
                text = "You can sprint by\nholding the Shift key";
            }

            game_state.overlay_system.set_pos_at(
                1,
                pos + glam::vec2(-constants::ENTITY_SIZE * 4., constants::ENTITY_SIZE * 4.),
            );
            game_state.overlay_system.set_rot_at(1, -constants::PI / 4.);

            game_state
                .overlay_system
                .set_pos_at(2, pos + glam::vec2(0., constants::ENTITY_SIZE * 3.));

            game_state.overlay_system.set_text_at(2, text);
        }
        2 => {
            let overlay_active = !game_state.guards_basic[0].is_dead();
            game_state.overlay_system.set_active_at(1, overlay_active);
            game_state.overlay_system.set_active_at(2, overlay_active);

            let mut pos =
                game_state.player.transform.position + glam::vec2(0., constants::ENTITY_SIZE * 3.);
            let mut text =
                "Target will usually have some guards\nYou can avoid them\nor eliminate them aswell\nBe wary\nguards can hear your footsteps";
            if (game_state.player.transform.position - glam::vec2(500., 460.)).length() > 50. {
                pos = game_state.guards_basic[0].guard.transform.position
                    + glam::vec2(0., constants::ENTITY_SIZE * 3.);
                text = "Try to eliminate this guard\nApproach silently by holding the Ctrl or C key\nEliminate with Q or E";
            }

            game_state.overlay_system.set_pos_at(
                1,
                pos + glam::vec2(-constants::ENTITY_SIZE * 4., constants::ENTITY_SIZE * 4.),
            );
            game_state.overlay_system.set_rot_at(1, -constants::PI / 4.);

            game_state
                .overlay_system
                .set_pos_at(2, pos + glam::vec2(0., constants::ENTITY_SIZE * 3.));

            game_state.overlay_system.set_text_at(2, text);
        }
        3 => {
            let overlay_active = !game_state.exit.player_exited;
            game_state.overlay_system.set_active_at(1, overlay_active);
            game_state.overlay_system.set_active_at(2, overlay_active);

            game_state.guards_basic[1].guard.dead_component.is_dead = true;

            let mut pos = game_state.guards_basic[1].guard.transform.position
                + glam::vec2(0., constants::ENTITY_SIZE * 3.);
            let mut text = "If a guard spots a dead body\nALL guards will be immediately alerted";
            if game_state.are_guards_alerted {
                pos = game_state.target.transform.position
                    + glam::vec2(0., constants::ENTITY_SIZE * 3.);
                text = "Try to bypass the guards\neliminate the target and escape";
            }

            game_state.overlay_system.set_pos_at(
                1,
                pos + glam::vec2(-constants::ENTITY_SIZE * 4., constants::ENTITY_SIZE * 4.),
            );
            game_state.overlay_system.set_rot_at(1, -constants::PI / 4.);

            game_state
                .overlay_system
                .set_pos_at(2, pos + glam::vec2(0., constants::ENTITY_SIZE * 3.));

            game_state.overlay_system.set_text_at(2, text);
        }
        4 => {
            if game_state.exit.player_exited {
                game_state.overlay_system.set_active_at(1, false);
                game_state.overlay_system.set_active_at(2, false);

                game_state.is_skip_tutorial = true;
                game_state.write_config(&util::config_filename());
            } else if game_state.target.is_dead()
                || (game_state.player.transform.position - glam::vec2(100., 380.)).length() < 30.
            {
                game_state.overlay_system.set_active_at(1, false);
                game_state.overlay_system.set_active_at(2, true);
            } else {
                game_state.overlay_system.set_active_at(2, false);
            }

            let mut pos = game_state.player.transform.position;
            let mut text =
                "Your second ability is stealth\nActivate it by holding the F key\nYou cannot move while being stealth\nTry to eliminate the guard";

            if game_state.target.is_dead() {
                pos = glam::vec2(645., 80.);
                text = "Remember, your score is determined\nby the amount of time needed\nfor completing each level\nso try to complete it as fast as possible";
            }

            game_state
                .overlay_system
                .set_pos_at(2, pos + glam::vec2(0., constants::ENTITY_SIZE * 3.));
            game_state.overlay_system.set_text_at(2, text);
        }
        _ => (),
    }
}
