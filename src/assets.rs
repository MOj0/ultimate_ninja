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

    pub dead: graphics::Image,

    pub wall: graphics::Image,
    pub box1: graphics::Image,
    pub box2: graphics::Image,

    pub teleport: graphics::Image,
    pub exit: graphics::Image,

    pub ultimate_ninja: graphics::Image,

    pub footstep: graphics::Image,

    pub floor1: graphics::Image,
    pub floor2: graphics::Image,
}

impl Assets {
    pub fn load(ctx: &mut Context, quad_ctx: &mut ggez::miniquad::GraphicsContext) -> Self {
        let stand = graphics::Image::new(ctx, quad_ctx, "textures/stand.png").unwrap();
        let walk_l1 = graphics::Image::new(ctx, quad_ctx, "textures/walk_l1.png").unwrap();
        let walk_l2 = graphics::Image::new(ctx, quad_ctx, "textures/walk_l2.png").unwrap();
        let walk_l3 = graphics::Image::new(ctx, quad_ctx, "textures/walk_l3.png").unwrap();
        let walk_l4 = graphics::Image::new(ctx, quad_ctx, "textures/walk_l4.png").unwrap();
        let walk_r1 = graphics::Image::new(ctx, quad_ctx, "textures/walk_r1.png").unwrap();
        let walk_r2 = graphics::Image::new(ctx, quad_ctx, "textures/walk_r2.png").unwrap();
        let walk_r3 = graphics::Image::new(ctx, quad_ctx, "textures/walk_r3.png").unwrap();
        let walk_r4 = graphics::Image::new(ctx, quad_ctx, "textures/walk_r4.png").unwrap();

        let dead = graphics::Image::new(ctx, quad_ctx, "textures/dead.png").unwrap();

        let wall = graphics::Image::new(ctx, quad_ctx, "textures/wall.png").unwrap();
        let box1 = graphics::Image::new(ctx, quad_ctx, "textures/box1.png").unwrap();
        let box2 = graphics::Image::new(ctx, quad_ctx, "textures/box2.png").unwrap();

        let teleport = graphics::Image::new(ctx, quad_ctx, "textures/oni.png").unwrap();
        let exit = graphics::Image::new(ctx, quad_ctx, "textures/exit.png").unwrap();

        let ultimate_ninja =
            graphics::Image::new(ctx, quad_ctx, "textures/ultimate_ninja.png").unwrap();

        let footstep = graphics::Image::new(ctx, quad_ctx, "textures/footstep.png").unwrap();

        let floor1 = graphics::Image::new(ctx, quad_ctx, "textures/floor1.png").unwrap();
        let floor2 = graphics::Image::new(ctx, quad_ctx, "textures/floor2.png").unwrap();

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

            dead,

            wall,
            box1,
            box2,

            teleport,
            exit,

            ultimate_ninja,

            footstep,

            floor1,
            floor2,
        }
    }
}
