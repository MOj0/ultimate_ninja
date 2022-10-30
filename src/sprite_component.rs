use ggez::miniquad;
use ggez::{graphics, Context, GameResult};

pub struct SpriteComponent {
    pub mesh_builder: graphics::MeshBuilder, // TODO: Change to Image once you have a spritesheet
}

impl SpriteComponent {
    pub fn new() -> Self {
        SpriteComponent {
            mesh_builder: graphics::MeshBuilder::new(),
        }
    }

    pub fn new_circle(
        &mut self,
        mode: graphics::DrawMode,
        point: glam::Vec2,
        radius: f32,
        tolerance: f32,
        color: graphics::Color,
    ) {
        self.mesh_builder = graphics::MeshBuilder::new();
        self.mesh_builder
            .circle(mode, point, radius, tolerance, color)
            .unwrap();
    }
    pub fn new_polygon(
        &mut self,
        mode: graphics::DrawMode,
        points: &Vec<glam::Vec2>,
        color: graphics::Color,
    ) {
        self.mesh_builder = graphics::MeshBuilder::new();
        self.mesh_builder.polygon(mode, points, color).unwrap();
    }
}

pub fn render(
    ctx: &mut Context,
    quad_ctx: &mut miniquad::Context,
    sprite: &SpriteComponent,
) -> GameResult {
    // TODO: For now everything is a mesh -> make a spritesheet and draw Image objects instead

    let mesh = sprite.mesh_builder.build(ctx, quad_ctx).unwrap();
    graphics::draw(ctx, quad_ctx, &mesh, graphics::DrawParam::default())
}
