pub struct MouseInputHandler {
    pub center: glam::Vec2,
    pub is_pressed: bool,
    pub circle_mesh: ggez::graphics::Mesh,
}

impl MouseInputHandler {
    pub fn new(
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::miniquad::GraphicsContext,
        center: glam::Vec2,
    ) -> Self {
        let circle_mesh = ggez::graphics::Mesh::new_circle(
            ctx,
            quad_ctx,
            ggez::graphics::DrawMode::fill(),
            center,
            20.,
            0.1,
            ggez::graphics::Color::BLUE,
        )
        .unwrap();

        Self {
            center,
            is_pressed: false,
            circle_mesh,
        }
    }

    #[inline]
    pub fn set_pressed(&mut self, is_pressed: bool) {
        self.is_pressed = is_pressed;
    }

    pub fn get_move_direction(&self, mouse_position: glam::Vec2) -> Option<glam::Vec2> {
        if !self.is_pressed {
            return None;
        }

        let diff = mouse_position - self.center;
        let diff_normalized = diff.normalize_or_zero();

        (diff.length_squared() < 1500.)
            .then_some(diff_normalized / 2.)
            .or_else(|| Some(diff_normalized))
    }
}
