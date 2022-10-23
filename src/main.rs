extern crate good_web_game as ggez;

use ggez::event::{KeyCode, KeyMods};
use ggez::input::MouseButton;
use ggez::miniquad;
use ggez::{audio, graphics};
use ggez::{Context, GameResult};

use std::f32::consts::PI;

use oorandom::Rand32;

// const WIDTH: u32 = 1024;
// const HEIGHT: u32 = 768;
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const TARGET_FPS: u32 = 60;

type Vec2 = glam::Vec2;

struct Player {
    size: (u16, u16),
    pos: Vec2,
    direction: Vec2,
    speed: f32,
}

impl Player {
    pub fn new(size: (u16, u16), pos: Vec2, speed: f32) -> Self {
        Player {
            size,
            pos,
            direction: glam::vec2(0., 0.),
            speed,
        }
    }

    /// Normalizes `direction` vector and sets it to `self.direction`
    pub fn set_direction(&mut self, direction: Vec2) {
        self.direction = direction / direction.length();
    }

    pub fn update(&mut self) {
        self.pos += self.direction * self.speed;

        self.direction = glam::vec2(0., 0.);
    }

    pub fn draw(
        &self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::Context,
    ) -> Result<(), ggez::GameError> {
        let circle = graphics::Mesh::new_circle(
            ctx,
            quad_ctx,
            graphics::DrawMode::fill(),
            self.pos,
            self.size.0 as f32,
            0.25,
            graphics::Color::BLACK,
        )
        .unwrap();

        graphics::draw(ctx, quad_ctx, &circle, graphics::DrawParam::default())
    }
}

struct GameState {
    rng: Rand32,
    player: Player,
}

impl GameState {
    pub fn new(ctx: &mut Context, quad_ctx: &mut miniquad::GraphicsContext) -> Self {
        let seed = 123456;
        let mut rng = oorandom::Rand32::new(seed);

        let player = Player::new((16, 16), glam::vec2(200., 200.), 4.);

        GameState { rng, player }
    }
}

impl ggez::event::EventHandler<ggez::GameError> for GameState {
    fn update(
        &mut self,
        _ctx: &mut Context,
        _quad_ctx: &mut miniquad::Context,
    ) -> Result<(), ggez::GameError> {
        self.player.update();

        Ok(())
    }

    fn draw(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::Context,
    ) -> Result<(), ggez::GameError> {
        let gray = graphics::Color::new(0.5, 0.5, 0.5, 1.);
        graphics::clear(ctx, quad_ctx, gray);

        self.player.draw(ctx, quad_ctx)?;

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
        let mut dir = glam::vec2(0., 0.);

        if keycode == KeyCode::W {
            dir += glam::vec2(0., -1.);
        }
        if keycode == KeyCode::S {
            dir += glam::vec2(0., 1.);
        }
        if keycode == KeyCode::A {
            dir += glam::vec2(-1., 0.);
        }
        if keycode == KeyCode::D {
            dir += glam::vec2(1., 0.);
        }

        self.player.set_direction(dir);
    }
}

fn main() -> GameResult {
    let conf = ggez::conf::Conf::default();

    ggez::start(conf, |mut context, mut quad_ctx| {
        Box::new(GameState::new(&mut context, &mut quad_ctx))
    })
}
