use crate::constants;
use ggez::miniquad;
use ggez::{graphics, Context, GameResult};

pub struct SpriteComponent {
    pub image: graphics::Image,
    pub color: graphics::Color,
}

impl SpriteComponent {
    pub fn new(image: graphics::Image, color: graphics::Color) -> Self {
        SpriteComponent { image, color }
    }

    pub fn set_color(&mut self, color: graphics::Color) {
        self.color = color;
    }
}

pub fn render(
    ctx: &mut Context,
    quad_ctx: &mut miniquad::Context,
    sprite: &SpriteComponent,
    draw_param: graphics::DrawParam,
) -> GameResult {
    graphics::draw(
        ctx,
        quad_ctx,
        &sprite.image,
        draw_param
            .scale(constants::SPRITE_SCALE)
            .offset(glam::vec2(0.5, 0.5))
            .color(sprite.color),
    )
}

pub fn render_mesh(
    ctx: &mut Context,
    quad_ctx: &mut miniquad::Context,
    mesh: &graphics::Mesh,
    draw_param: graphics::DrawParam,
) -> GameResult {
    graphics::draw(ctx, quad_ctx, mesh, draw_param)
}
