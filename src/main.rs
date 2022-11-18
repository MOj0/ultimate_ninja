mod animation_component;
mod assets;
mod collision_component;
mod constants;
mod entities;
mod level;
mod look_component;
mod mouse_input_handler;
mod move_component;
mod sound_collection;
mod sprite_component;
mod stamina_component;
mod teleport_component;
mod transform_component;
mod util;

use crate::assets::Assets;
use crate::mouse_input_handler::MouseInputHandler;
use crate::sound_collection::SoundCollection;
use crate::sprite_component::SpriteComponent;

extern crate good_web_game as ggez;

use ggez::event::{KeyCode, KeyMods};
use ggez::input::MouseButton;
use ggez::miniquad;
use ggez::{audio, graphics, Context, GameResult};

use oorandom::Rand32;

pub struct GameState {
    rng: Rand32,
    assets: assets::Assets,
    player: entities::player::Player,
    target: entities::target::Target,
    guards: Vec<entities::guard::Guard>,
    walls: Vec<entities::wall::Wall>,
    exit: entities::exit::Exit,
    double_press_timer: Option<f32>,
    sound_collection: SoundCollection,
    mouse_input_handler: MouseInputHandler,
    level_idx: usize,
    debug_draw: bool,
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

        let exit = entities::exit::Exit::new(
            glam::Vec2::default(),
            SpriteComponent::new(assets.exit.clone(), ggez::graphics::Color::WHITE),
        );

        let mut sounds = [
            audio::Source::new(ctx, "sounds/stealth.ogg").unwrap(),
            audio::Source::new(ctx, "sounds/unstealth.ogg").unwrap(),
            audio::Source::new(ctx, "sounds/teleport_initial.ogg").unwrap(),
            audio::Source::new(ctx, "sounds/teleport.ogg").unwrap(),
            audio::Source::new(ctx, "sounds/player_dead.ogg").unwrap(),
            audio::Source::new(ctx, "sounds/target_killed.ogg").unwrap(),
            audio::Source::new(ctx, "sounds/dead_target_detected.ogg").unwrap(),
            audio::Source::new(ctx, "sounds/level_exit.ogg").unwrap(),
        ];
        sounds
            .iter_mut()
            .for_each(|sound| sound.set_volume(ctx, 0.5).unwrap());

        let sound_collection = SoundCollection {
            sounds,
            is_on: true,
        };

        let mouse_input_handler = MouseInputHandler::new(
            ctx,
            quad_ctx,
            glam::vec2(
                (constants::WIDTH - 125) as f32,
                (constants::HEIGHT - 125) as f32,
            ),
            80.,
        );

        let level_idx = 0;

        let mut game_state = GameState {
            rng,
            assets,
            player,
            target,
            guards: vec![],
            walls: vec![],
            exit,
            double_press_timer: None,
            sound_collection,
            mouse_input_handler,
            level_idx,
            debug_draw: false,
        };

        level::load_level(ctx, quad_ctx, &mut game_state, level_idx);

        game_state
    }

    pub fn reset(&mut self) {
        self.player.is_detected = false;
        self.player.is_stealth = false;
        self.player.stealth_intent = false;

        self.target.is_dead = false;

        self.exit.player_exited = false;

        self.guards.clear();
        self.walls.clear();
    }
}

impl ggez::event::EventHandler<ggez::GameError> for GameState {
    fn update(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::Context,
    ) -> Result<(), ggez::GameError> {
        let dt = ggez::timer::delta(ctx).as_secs_f32();

        mouse_input_handler::system(self, ggez::timer::time_since_start(ctx).as_secs_f32());

        entities::wall::check_collision(self);

        entities::player::system(ctx, self, dt);

        self.target.update(dt);

        entities::guard::system(ctx, self, dt);

        look_component::system(self);

        entities::exit::system(self, dt);

        if self.exit.player_exited {
            self.level_idx += 1;
            level::load_level(ctx, quad_ctx, self, self.level_idx);
        }

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

        if self.player.teleport.location.is_some() {
            sprite_component::render_sprite(
                ctx,
                quad_ctx,
                &self.player.teleport.sprite,
                DrawParam::default().dest(self.player.teleport.location.as_ref().unwrap().position),
            )?;
        }

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
            &self.target.get_curr_animation_frame(),
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

        if self.debug_draw {
            // Draw look rays
            self.guards
                .iter()
                .map(|guard| {
                    guard
                        .look
                        .ray_lines
                        .iter()
                        .enumerate()
                        .map(|(ray_idx, ray_line)| {
                            sprite_component::render_mesh(
                                ctx,
                                quad_ctx,
                                ray_line,
                                DrawParam::default()
                                    .dest(guard.transform.position)
                                    .rotation(
                                        -constants::PI / 2.
                                            - util::get_vec_angle(guard.look.look_at),
                                    )
                                    .scale(glam::Vec2::splat(guard.look.ray_scales[ray_idx])),
                            )
                        })
                        .count();
                })
                .count();
        }

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

        sprite_component::render_mesh(
            ctx,
            quad_ctx,
            &self.mouse_input_handler.touch_area,
            DrawParam::default(),
        )
        .unwrap();

        sprite_component::render_mesh(
            ctx,
            quad_ctx,
            &self.mouse_input_handler.direction_circle,
            DrawParam::default().offset(-self.mouse_input_handler.direction_offset),
        )
        .unwrap();

        if self.target.is_dead {
            sprite_component::render_sprite(
                ctx,
                quad_ctx,
                &self.exit.sprite,
                DrawParam::default()
                    .dest(self.exit.transform.position)
                    .rotation(-self.exit.scale_rotation_counter),
            )?;
        }

        if self.debug_draw {
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
            graphics::draw(
                ctx,
                quad_ctx,
                &util::make_text(format!("player_exited: {}", self.exit.player_exited), 24.),
                DrawParam::from((glam::vec2(4., 80.),)),
            )?;
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        _quad_ctx: &mut miniquad::Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        repeat: bool,
    ) {
        if keycode != KeyCode::F {
            self.double_press_timer = None;
        }

        match keycode {
            KeyCode::W => self.player.set_y_dir(-1.),
            KeyCode::S => self.player.set_y_dir(1.),
            KeyCode::A => self.player.set_x_dir(-1.),
            KeyCode::D => self.player.set_x_dir(1.),
            KeyCode::F => {
                let curr_t = ggez::timer::time_since_start(ctx).as_secs_f32();

                if repeat {
                    self.player.set_stealth_intent(true);
                } else if curr_t - self.double_press_timer.unwrap_or(-1.)
                    < constants::DOUBLE_PRESS_TIME
                {
                    self.player.teleport_action(ctx, &mut self.sound_collection);
                    self.double_press_timer = None;
                } else {
                    self.double_press_timer = Some(curr_t);
                }
            }
            KeyCode::M => self.sound_collection.is_on = !self.sound_collection.is_on,
            KeyCode::B => self.debug_draw = !self.debug_draw,
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

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        _quad_ctx: &mut miniquad::GraphicsContext,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        match button {
            MouseButton::Left => {
                let curr_t = ggez::timer::time_since_start(ctx).as_secs_f32();

                // NOTE: If is_pressed is set to true, there is no PlayerAction to handle
                self.mouse_input_handler.handle_pressed(true, curr_t);
            }
            _ => (),
        }
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        _quad_ctx: &mut miniquad::Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        match button {
            MouseButton::Left => {
                let curr_t = ggez::timer::time_since_start(ctx).as_secs_f32();

                match self.mouse_input_handler.handle_pressed(false, curr_t) {
                    Some(mouse_input_handler::PlayerAblity::Teleport) => {
                        self.player.teleport_action(ctx, &mut self.sound_collection)
                    }
                    Some(mouse_input_handler::PlayerAblity::Stealth(is_stealth)) => {
                        self.player.set_stealth_intent(is_stealth)
                    }
                    None => (),
                }
            }
            _ => (),
        }
    }

    fn mouse_motion_event(
        &mut self,
        ctx: &mut Context,
        _quad_ctx: &mut miniquad::Context,
        x: f32,
        y: f32,
        _dx: f32,
        _dy: f32,
    ) {
        let dir = self
            .mouse_input_handler
            .get_move_direction(glam::vec2(x, y));

        self.player.set_dir(dir.unwrap_or_default());

        self.player.set_stealth_intent(false);
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
