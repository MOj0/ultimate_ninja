mod animation_component;
mod assets;
mod collision_component;
mod constants;
mod entities;
mod level;
mod look_component;
mod move_component;
mod sprite_component;
mod stamina_component;
mod transform_component;
mod util;

use crate::assets::Assets;
use crate::sprite_component::SpriteComponent;

extern crate good_web_game as ggez;

use ggez::event::{KeyCode, KeyMods};
use ggez::input::MouseButton;
use ggez::miniquad;
use ggez::{audio, graphics};
use ggez::{Context, GameResult};

use oorandom::Rand32;

pub struct GameState {
    rng: Rand32,
    player: entities::player::Player,
    target: entities::target::Target,
    guards: Vec<entities::guard::Guard>,
    walls: Vec<entities::wall::Wall>,
}

impl GameState {
    pub fn new(ctx: &mut Context, quad_ctx: &mut miniquad::GraphicsContext) -> Self {
        let seed = 123456;
        let mut rng = oorandom::Rand32::new(seed);

        let assets = Assets::load(ctx, quad_ctx);

        let player = entities::player::Player::new(
            ctx,
            quad_ctx,
            glam::Vec2::default(),
            &assets,
            ggez::graphics::Color::BLACK,
        );

        let target = entities::target::Target::new(
            glam::Vec2::default(),
            &assets,
            ggez::graphics::Color::GREEN,
        );

        let mut game_state = GameState {
            rng,
            player,
            target,
            guards: vec![],
            walls: vec![],
        };

        level::load_level(ctx, quad_ctx, &mut game_state, &assets, 0);

        game_state
    }
}

impl ggez::event::EventHandler<ggez::GameError> for GameState {
    fn update(
        &mut self,
        ctx: &mut Context,
        _quad_ctx: &mut miniquad::Context,
    ) -> Result<(), ggez::GameError> {
        let dt = ggez::timer::delta(ctx).as_secs_f32();

        entities::wall::check_collision(self);

        entities::player::system(self, dt);

        self.target.update(dt);

        entities::guard::system(self, dt);

        look_component::system(self);

        Ok(())
    }

    fn draw(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::Context,
    ) -> Result<(), ggez::GameError> {
        let gray = graphics::Color::new(0.5, 0.5, 0.5, 1.);
        graphics::clear(ctx, quad_ctx, gray);

        use graphics::DrawParam;

        sprite_component::render_sprite(
            ctx,
            quad_ctx,
            &self.player.animation.get_curr_frame(),
            DrawParam::default()
                .dest(self.player.transform.position)
                .rotation(-self.player.transform.angle),
        )?;

        sprite_component::render_sprite(
            ctx,
            quad_ctx,
            &self.target.animation.get_curr_frame(),
            DrawParam::default()
                .dest(self.target.transform.position)
                .rotation(-self.target.transform.angle),
        )?;

        self.guards
            .iter()
            .map(|guard| {
                sprite_component::render_sprite(
                    ctx,
                    quad_ctx,
                    &guard.animation.get_curr_frame(),
                    DrawParam::default()
                        .dest(guard.transform.position)
                        .rotation(-guard.transform.angle),
                )
            })
            .count();

        // Draw look mesh compositions
        self.guards
            .iter()
            .map(|guard| {
                guard
                    .look
                    .fov_mesh_composition
                    .iter()
                    .zip(&guard.look.ray_scales)
                    .flat_map(|(fov_section, scale)| {
                        sprite_component::render_mesh(
                            ctx,
                            quad_ctx,
                            fov_section,
                            DrawParam::default()
                                .dest(guard.transform.position)
                                .rotation(
                                    -constants::PI / 2. - util::get_vec_angle(guard.look.look_at),
                                )
                                .scale(glam::Vec2::splat(*scale)),
                        )
                    })
                    .count()
            })
            .count();

        // TODO: Only draw this in 'debug' mode
        // // Draw look rays
        // self.guards
        //     .iter()
        //     .map(|guard| {
        //         guard
        //             .look
        //             .ray_lines
        //             .iter()
        //             .enumerate()
        //             .map(|(ray_idx, ray_line)| {
        //                 sprite_component::render_mesh(
        //                     ctx,
        //                     quad_ctx,
        //                     ray_line,
        //                     DrawParam::default()
        //                         .dest(guard.transform.position)
        //                         .rotation(
        //                             -constants::PI / 2. - util::get_vec_angle(guard.look.look_at),
        //                         )
        //                         .scale(glam::Vec2::splat(guard.look.ray_scales[ray_idx])),
        //                 )
        //             })
        //             .count();
        //     })
        //     .count();

        self.walls
            .iter()
            .map(|wall| {
                sprite_component::render(
                    ctx,
                    quad_ctx,
                    &wall.sprite,
                    DrawParam::default()
                        .dest(wall.transform.position)
                        .scale(wall.sprite.scale),
                )
            })
            .count();

        sprite_component::render_mesh(
            ctx,
            quad_ctx,
            &self.player.stamina.stamina_mesh,
            DrawParam::default().scale(glam::vec2(self.player.stamina.get_percentage(), 1.)),
        )
        .unwrap();

        // TODO: only draw this in 'debug' mode
        graphics::draw(
            ctx,
            quad_ctx,
            &util::make_text(format!("is_target_dead: {}", self.target.is_dead), 24.),
            DrawParam::from((glam::vec2(4., 32.),)),
        )?;
        graphics::draw(
            ctx,
            quad_ctx,
            &util::make_text(
                format!("is_player_detected: {}", self.player.is_detected),
                24.,
            ),
            DrawParam::from((glam::vec2(4., 56.),)),
        )?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _quad_ctx: &mut miniquad::Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        repeat: bool,
    ) {
        match keycode {
            KeyCode::W => self.player.set_y_dir(-1.),
            KeyCode::S => self.player.set_y_dir(1.),
            KeyCode::A => self.player.set_x_dir(-1.),
            KeyCode::D => self.player.set_x_dir(1.),
            KeyCode::F => {
                if repeat {
                    self.player.set_stealth_intent(true);
                }
            }
            _ => (),
        }
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        _quad_ctx: &mut miniquad::Context,
        keycode: KeyCode,
        _keymods: KeyMods,
    ) {
        if keycode == KeyCode::W && self.player.move_component.direction.y < 0. {
            self.player.set_y_dir(0.);
        } else if keycode == KeyCode::S && self.player.move_component.direction.y > 0. {
            self.player.set_y_dir(0.);
        } else if keycode == KeyCode::A && self.player.move_component.direction.x < 0. {
            self.player.set_x_dir(0.);
        } else if keycode == KeyCode::D && self.player.move_component.direction.x > 0. {
            self.player.set_x_dir(0.);
        } else if keycode == KeyCode::F {
            self.player.set_stealth_intent(false);
        }
    }
}

fn main() -> GameResult {
    let resource_dir = std::path::PathBuf::from("./resources");

    let conf = ggez::conf::Conf::default()
        .window_title("Ultimate Ninja".to_owned())
        .physical_root_dir(Some(resource_dir));

    ggez::start(conf, |mut context, mut quad_ctx| {
        Box::new(GameState::new(&mut context, &mut quad_ctx))
    })
}
