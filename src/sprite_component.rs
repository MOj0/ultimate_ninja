use ggez::miniquad;
use ggez::{graphics, Context, GameResult};

pub struct SpriteComponent {
    pub image: graphics::Image,
    pub color: graphics::Color,
    pub scale: glam::Vec2,
}

impl SpriteComponent {
    pub fn new(image: graphics::Image, color: graphics::Color) -> Self {
        SpriteComponent {
            image,
            color,
            scale: glam::vec2(1., 1.),
        }
    }

    pub fn scale(mut self, scale: glam::Vec2) -> Self {
        self.scale = scale;
        self
    }

    #[inline]
    pub fn set_color(&mut self, color: graphics::Color) {
        self.color = color;
    }

    #[inline]
    pub fn set_scale(&mut self, scale: f32) {
        self.scale = glam::Vec2::splat(scale);
    }
}

#[inline]
pub fn render(
    ctx: &mut Context,
    quad_ctx: &mut miniquad::Context,
    sprite: &SpriteComponent,
    draw_param: graphics::DrawParam,
) -> GameResult {
    graphics::draw(ctx, quad_ctx, &sprite.image, draw_param)
}

pub fn render_sprite(
    ctx: &mut Context,
    quad_ctx: &mut miniquad::Context,
    sprite: &SpriteComponent,
    draw_param: graphics::DrawParam,
) -> GameResult {
    render(
        ctx,
        quad_ctx,
        sprite,
        draw_param
            .scale(sprite.scale)
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
