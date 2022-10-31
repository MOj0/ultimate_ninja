use ggez::{graphics, Context};

pub struct Assets {
    pub stand: graphics::Image,
    pub walk_l1: graphics::Image,
    pub walk_l2: graphics::Image,
    pub walk_l3: graphics::Image,
    pub walk_l4: graphics::Image,
    pub walk_r1: graphics::Image,
    pub walk_r2: graphics::Image,
    pub walk_r3: graphics::Image,
    pub walk_r4: graphics::Image,
    pub box1: graphics::Image,
    pub box2: graphics::Image,
}

impl Assets {
    pub fn load(ctx: &mut Context, quad_ctx: &mut ggez::miniquad::GraphicsContext) -> Self {
        let stand = graphics::Image::new(ctx, quad_ctx, "/stand.png").unwrap();
        let walk_l1 = graphics::Image::new(ctx, quad_ctx, "/walk_l1.png").unwrap();
        let walk_l2 = graphics::Image::new(ctx, quad_ctx, "/walk_l2.png").unwrap();
        let walk_l3 = graphics::Image::new(ctx, quad_ctx, "/walk_l3.png").unwrap();
        let walk_l4 = graphics::Image::new(ctx, quad_ctx, "/walk_l4.png").unwrap();
        let walk_r1 = graphics::Image::new(ctx, quad_ctx, "/walk_r1.png").unwrap();
        let walk_r2 = graphics::Image::new(ctx, quad_ctx, "/walk_r2.png").unwrap();
        let walk_r3 = graphics::Image::new(ctx, quad_ctx, "/walk_r3.png").unwrap();
        let walk_r4 = graphics::Image::new(ctx, quad_ctx, "/walk_r4.png").unwrap();

        let box1 = graphics::Image::new(ctx, quad_ctx, "/box1.png").unwrap();
        let box2 = graphics::Image::new(ctx, quad_ctx, "/box2.png").unwrap();

        Assets {
            stand,
            walk_l1,
            walk_l2,
            walk_l3,
            walk_l4,
            walk_r1,
            walk_r2,
            walk_r3,
            walk_r4,
            box1,
            box2,
        }
    }
}
