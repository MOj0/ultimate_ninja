mod animation_component;
mod assets;
mod collision_component;
mod constants;
mod entities;
mod look_component;
mod move_component;
mod sprite_component;
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
    assets: Assets,
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

        let player_animation =
            util::build_walk_animation(&assets, 0.05, ggez::graphics::Color::BLACK);
        let player = entities::player::Player::new(glam::vec2(200., 400.), player_animation);

        let target_animation =
            util::build_walk_animation(&assets, 0.05, ggez::graphics::Color::GREEN);
        let target = entities::target::Target::new(glam::vec2(500., 300.), target_animation);

        let guard_animation = util::build_walk_animation(&assets, 0.05, ggez::graphics::Color::RED);
        let guard1 =
            entities::guard::Guard::new(ctx, quad_ctx, glam::vec2(400., 400.), guard_animation);
        let guards = vec![guard1];

        let box1_sprite = SpriteComponent::new(assets.box1.clone(), ggez::graphics::Color::WHITE);
        let box1 = entities::wall::Wall::new(glam::vec2(100., 100.), 200., 100., box1_sprite);
        let box2_sprite = SpriteComponent::new(assets.box2.clone(), ggez::graphics::Color::WHITE);
        let box2 = entities::wall::Wall::new(glam::vec2(80., 200.), 100., 400., box2_sprite);
        let walls = vec![box1, box2];

        GameState {
            rng,
            assets,
            player,
            target,
            guards,
            walls,
        }
    }
}

impl ggez::event::EventHandler<ggez::GameError> for GameState {
    fn update(
        &mut self,
        ctx: &mut Context,
        _quad_ctx: &mut miniquad::Context,
    ) -> Result<(), ggez::GameError> {
        let dt = ggez::timer::delta(ctx).as_secs_f32();

        entities::wall::system(self);

        entities::player::system(self, dt);

        self.target.update(dt);

        entities::guard::system(self, dt);

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
                .rotation(-util::get_vec_angle(self.player.move_component.direction)),
        )?;

        sprite_component::render_sprite(
            ctx,
            quad_ctx,
            &self.target.animation.get_curr_frame(),
            DrawParam::default()
                .dest(self.target.transform.position)
                .rotation(-util::get_vec_angle(self.target.move_component.direction)),
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
                        .rotation(-util::get_vec_angle(guard.move_component.direction)),
                )
            })
            .count();

        self.guards
            .iter()
            .map(|guard| {
                sprite_component::render_mesh(
                    ctx,
                    quad_ctx,
                    &guard.look.fov_mesh,
                    DrawParam::default()
                        .dest(guard.transform.position)
                        .rotation(-constants::PI / 2. - util::get_vec_angle(guard.look.look_at)),
                )
            })
            .count();

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

        graphics::draw(
            ctx,
            quad_ctx,
            &util::make_text(format!("is_target_dead: {}", self.target.is_dead), 24.),
            DrawParam::from((glam::vec2(4., 8.),)),
        )?;
        graphics::draw(
            ctx,
            quad_ctx,
            &util::make_text(
                format!("is_player_detected: {}", self.player.is_detected),
                24.,
            ),
            DrawParam::from((glam::vec2(4., 32.),)),
        )?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _quad_ctx: &mut miniquad::Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::W => self.player.set_y_dir(-1.),
            KeyCode::S => self.player.set_y_dir(1.),
            KeyCode::A => self.player.set_x_dir(-1.),
            KeyCode::D => self.player.set_x_dir(1.),
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
