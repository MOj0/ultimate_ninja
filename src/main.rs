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

struct Entity {
    size: f32,
    pos: Vec2,
    direction: Vec2,
    speed: f32,
}

impl Entity {
    pub fn new(size: f32, pos: Vec2, speed: f32) -> Self {
        Entity {
            size,
            pos,
            direction: glam::vec2(0., 0.),
            speed,
        }
    }

    /// Normalizes `direction` vector and sets it to `self.direction`
    pub fn set_direction(&mut self, direction: Vec2) {
        let len_sq = direction.length_squared();

        if len_sq == 0. {
            self.direction = direction;
            return;
        }

        self.direction = direction / len_sq.sqrt();
    }

    pub fn update(&mut self) {
        self.pos += self.direction * self.speed;
    }

    pub fn draw(
        &self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::Context,
        color: graphics::Color,
    ) -> Result<(), ggez::GameError> {
        let circle = graphics::Mesh::new_circle(
            ctx,
            quad_ctx,
            graphics::DrawMode::fill(),
            self.pos,
            self.size,
            0.25,
            color,
        )
        .unwrap();

        graphics::draw(ctx, quad_ctx, &circle, graphics::DrawParam::default())
    }
}

struct Player {
    entity: Entity,
    x_dir: f32,
    y_dir: f32,
}

impl Player {
    pub fn new(size: f32, pos: Vec2, speed: f32) -> Self {
        let entity = Entity::new(size, pos, speed);
        Player {
            entity,
            x_dir: 0.0,
            y_dir: 0.0,
        }
    }

    pub fn set_x_dir(&mut self, x_dir: f32) {
        self.x_dir = x_dir;
    }

    pub fn set_y_dir(&mut self, y_dir: f32) {
        self.y_dir = y_dir;
    }

    pub fn update(&mut self) {
        self.entity
            .set_direction(glam::vec2(self.x_dir, self.y_dir));

        self.entity.update();
    }

    pub fn draw(
        &self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::Context,
        color: graphics::Color,
    ) -> Result<(), ggez::GameError> {
        self.entity.draw(ctx, quad_ctx, color)
    }
}

struct Target {
    entity: Entity,
    is_dead: bool,
}

impl Target {
    pub fn new(size: f32, pos: Vec2, speed: f32) -> Self {
        let entity = Entity::new(size, pos, speed);

        Target {
            entity,
            is_dead: false,
        }
    }

    pub fn update(&mut self) {
        self.entity.set_direction(glam::vec2(-1., 0.));

        self.entity.update();
    }

    pub fn draw(
        &self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::Context,
    ) -> Result<(), ggez::GameError> {
        self.entity.draw(ctx, quad_ctx, graphics::Color::GREEN)
    }
}

struct Guard {
    entity: Entity,
    look_at: Vec2,
    fov: f32,
    view_distance: f32,
    tmp_counter: f32,
}

impl Guard {
    pub fn new(size: f32, pos: Vec2, speed: f32) -> Self {
        let entity = Entity::new(size, pos, speed);

        Guard {
            entity,
            look_at: glam::vec2(0., 0.),
            fov: PI / 6.,
            view_distance: 200.,
            tmp_counter: 0.,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.tmp_counter += dt;
        self.entity
            .set_direction(glam::vec2(self.tmp_counter.cos(), -self.tmp_counter.sin()));
        self.look_at = self.entity.direction;

        self.entity.update();
    }

    pub fn draw(
        &self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::Context,
    ) -> Result<(), ggez::GameError> {
        self.entity.draw(ctx, quad_ctx, graphics::Color::RED)?;

        let line_of_sight = self.entity.pos + self.look_at * self.view_distance;
        let fov_corner1 = rotate_point_around_center(line_of_sight, self.entity.pos, -self.fov);
        let fov_corner2 = rotate_point_around_center(line_of_sight, self.entity.pos, self.fov);
        let intersection = (fov_corner1 - fov_corner2) / 2. + fov_corner2;

        let arc_points = get_arc_points(
            intersection,
            glam::vec2(
                ((fov_corner1 - fov_corner2) / 2.).length(),
                (line_of_sight - intersection).length(),
            ),
            0.,
            PI,
            -PI / 2. - self.look_at.angle_between(glam::vec2(1., 0.)),
        );

        let mut fov_points = arc_points;
        fov_points.insert(0, fov_corner1);
        fov_points.insert(0, self.entity.pos);
        fov_points.push(fov_corner2);

        let fov_polygon = graphics::Mesh::new_polygon(
            ctx,
            quad_ctx,
            graphics::DrawMode::fill(),
            &fov_points,
            graphics::Color {
                r: 0.5,
                g: 0.,
                b: 0.,
                a: 0.5,
            },
        )
        .unwrap();

        graphics::draw(ctx, quad_ctx, &fov_polygon, graphics::DrawParam::default())?;

        Ok(())
    }
}

struct GameState {
    rng: Rand32,
    player: Player,
    target: Target,
    guards: Vec<Guard>,
    tmp_player_color: graphics::Color,
}

impl GameState {
    pub fn new(ctx: &mut Context, quad_ctx: &mut miniquad::GraphicsContext) -> Self {
        let seed = 123456;
        let mut rng = oorandom::Rand32::new(seed);

        let player = Player::new(10., glam::vec2(200., 200.), 4.);

        let target = Target::new(10., glam::vec2(500., 300.), 0.1);

        let guard1 = Guard::new(10., glam::vec2(400., 400.), 0.2);
        let guards = vec![guard1];

        GameState {
            rng,
            player,
            target,
            guards,
            tmp_player_color: graphics::Color::BLACK,
        }
    }

    fn check_player_target_collision(&self) -> bool {
        let distance_vec = self.player.entity.pos - self.target.entity.pos;

        distance_vec.length() <= self.player.entity.size + self.target.entity.size
    }

    fn is_player_detected(&self) -> bool {
        for guard in self.guards.iter() {
            let vec_to_player = self.player.entity.pos - guard.entity.pos;
            let len_to_player_sq = vec_to_player.length_squared();

            // Player is colliding with the guard
            if len_to_player_sq <= (self.player.entity.size + guard.entity.size).powi(2) {
                return true;
            }

            if len_to_player_sq > guard.view_distance.powi(2) {
                continue;
            }

            // Guard spots the player
            let dot = guard.look_at.dot(vec_to_player);
            let angle = (dot / (guard.look_at.length() * len_to_player_sq.sqrt())).acos();
            if angle <= guard.fov {
                return true;
            }
        }
        false
    }
}

impl ggez::event::EventHandler<ggez::GameError> for GameState {
    fn update(
        &mut self,
        _ctx: &mut Context,
        _quad_ctx: &mut miniquad::Context,
    ) -> Result<(), ggez::GameError> {
        let dt = 1. / TARGET_FPS as f32;

        self.guards.iter_mut().for_each(|guard| guard.update(dt));
        self.target.update();

        self.player.update();

        if self.check_player_target_collision() {
            self.target.is_dead = true;
        }

        if self.is_player_detected() {
            self.tmp_player_color = graphics::Color::BLUE;
        } else {
            self.tmp_player_color = graphics::Color::BLACK;
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

        self.player.draw(ctx, quad_ctx, self.tmp_player_color)?;

        self.target.draw(ctx, quad_ctx)?;

        self.guards
            .iter()
            .map(|guard| guard.draw(ctx, quad_ctx))
            .count();

        graphics::draw(
            ctx,
            quad_ctx,
            &make_text(format!("direction: {}", self.player.entity.direction), 24.),
            graphics::DrawParam::from((glam::vec2(4., 8.),)),
        )?;
        graphics::draw(
            ctx,
            quad_ctx,
            &make_text(format!("is_target_dead: {}", self.target.is_dead), 24.),
            graphics::DrawParam::from((glam::vec2(4., 32.),)),
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
        if keycode == KeyCode::W && self.player.y_dir < 0. {
            self.player.set_y_dir(0.);
        } else if keycode == KeyCode::S && self.player.y_dir > 0. {
            self.player.set_y_dir(0.);
        } else if keycode == KeyCode::A && self.player.x_dir < 0. {
            self.player.set_x_dir(0.);
        } else if keycode == KeyCode::D && self.player.x_dir > 0. {
            self.player.set_x_dir(0.);
        }
    }
}

fn make_text(s: String, scale: f32) -> graphics::Text {
    graphics::Text::new(graphics::TextFragment::new(s).scale(scale))
}

fn rotate_point_around_center(point: Vec2, center: Vec2, angle: f32) -> Vec2 {
    let r_x = angle.cos() * (point.x - center.x) - angle.sin() * (point.y - center.y) + center.x;
    let r_y = angle.sin() * (point.x - center.x) + angle.cos() * (point.y - center.y) + center.y;

    glam::vec2(r_x, r_y)
}

fn get_arc_points(
    center: Vec2,
    radii: Vec2,
    start_angle: f32,
    sweep_angle: f32,
    x_rotation: f32,
) -> Vec<Vec2> {
    let arc = lyon_geom::Arc {
        center: lyon_geom::euclid::Point2D {
            x: center.x,
            y: center.y,
            _unit: std::marker::PhantomData,
        },
        radii: lyon_geom::euclid::Vector2D {
            x: radii.x,
            y: radii.y,
            _unit: std::marker::PhantomData,
        },
        start_angle: lyon_geom::Angle::radians(start_angle),
        sweep_angle: lyon_geom::Angle::radians(sweep_angle),
        x_rotation: lyon_geom::Angle::radians(x_rotation),
    };

    arc.flattened(0.01)
        .into_iter()
        .map(|p| glam::vec2(p.x, p.y))
        .collect::<Vec<Vec2>>()
}

fn main() -> GameResult {
    let conf = ggez::conf::Conf::default().window_title("Ultimate Ninja".to_owned());

    ggez::start(conf, |mut context, mut quad_ctx| {
        Box::new(GameState::new(&mut context, &mut quad_ctx))
    })
}
