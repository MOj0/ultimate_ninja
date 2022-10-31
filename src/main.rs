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

extern crate good_web_game as ggez;

use ggez::event::{KeyCode, KeyMods};
use ggez::input::MouseButton;
use ggez::miniquad;
use ggez::{audio, graphics};
use ggez::{Context, GameResult};

use oorandom::Rand32;

// const WIDTH: u32 = 1024;
// const HEIGHT: u32 = 768;
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub struct GameState {
    rng: Rand32,
    assets: Assets,
    player: entities::player::Player,
    target: entities::target::Target,
    guards: Vec<entities::guard::Guard>,
}

impl GameState {
    pub fn new(ctx: &mut Context, quad_ctx: &mut miniquad::GraphicsContext) -> Self {
        let seed = 123456;
        let mut rng = oorandom::Rand32::new(seed);

        let assets = Assets::load(ctx, quad_ctx);

        let player = entities::player::Player::new(glam::vec2(200., 200.), 8., 4., &assets);

        let target = entities::target::Target::new(glam::vec2(500., 300.), 8., 0.1, &assets);
        let guard1 = entities::guard::Guard::new(
            ctx,
            quad_ctx,
            glam::vec2(400., 400.),
            8.,
            0.2,
            constants::PI / 6.,
            200.,
            &assets,
        );
        let guards = vec![guard1];

        GameState {
            rng,
            assets,
            player,
            target,
            guards,
        }
    }
}

impl ggez::event::EventHandler<ggez::GameError> for GameState {
    fn update(
        &mut self,
        ctx: &mut Context,
        _quad_ctx: &mut miniquad::Context,
    ) -> Result<(), ggez::GameError> {
        entities::player::system(self);

        entities::guard::system(self, ctx);

        self.target.update_movement();

        Ok(())
    }

    fn draw(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::Context,
    ) -> Result<(), ggez::GameError> {
        let gray = graphics::Color::new(0.5, 0.5, 0.5, 1.);
        graphics::clear(ctx, quad_ctx, gray);

        use ggez::graphics::DrawParam;

        sprite_component::render(
            ctx,
            quad_ctx,
            &self.player.sprite,
            DrawParam::from((self.player.transform.position,)),
        )?;
        sprite_component::render(
            ctx,
            quad_ctx,
            &self.target.sprite,
            DrawParam::from((self.target.transform.position,)),
        )?;

        self.guards
            .iter()
            .map(|guard| {
                sprite_component::render(
                    ctx,
                    quad_ctx,
                    &guard.sprite,
                    DrawParam::from((guard.transform.position,)),
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
                        .rotation(
                            -constants::PI / 2.
                                - guard.look.look_at.angle_between(glam::vec2(1., 0.)),
                        ),
                )
            })
            .count();

        graphics::draw(
            ctx,
            quad_ctx,
            &util::make_text(format!("is_target_dead: {}", self.target.is_dead), 24.),
            graphics::DrawParam::from((glam::vec2(4., 8.),)),
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
        // .window_width(WIDTH as i32)
        // .window_height(HEIGHT as i32)
        .physical_root_dir(Some(resource_dir));

    ggez::start(conf, |mut context, mut quad_ctx| {
        Box::new(GameState::new(&mut context, &mut quad_ctx))
    })
}
