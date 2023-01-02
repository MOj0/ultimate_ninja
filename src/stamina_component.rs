pub struct StaminaComponent {
    pub max_stamina: f32,
    pub stamina: f32,
    pub decrease_rate: f32,
    pub increase_rate: f32,
    pub stamina_mesh: ggez::graphics::Mesh,
}

impl StaminaComponent {
    pub fn new(
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::miniquad::GraphicsContext,
        max_stamina: f32,
        decrease_rate: f32,
        increase_rate: f32,
        rect: ggez::graphics::Rect,
        color: ggez::graphics::Color,
    ) -> Self {
        Self {
            max_stamina,
            stamina: max_stamina,
            decrease_rate,
            increase_rate,
            stamina_mesh: ggez::graphics::Mesh::new_rectangle(
                ctx,
                quad_ctx,
                ggez::graphics::DrawMode::fill(),
                rect,
                color,
            )
            .unwrap(),
        }
    }

    #[inline]
    pub fn update(&mut self, is_decreasing: bool) {
        self.stamina = (self.stamina
            + is_decreasing
                .then(|| -self.decrease_rate)
                .unwrap_or(self.increase_rate))
        .clamp(0., self.max_stamina);
    }

    #[inline]
    pub fn get_percentage(&self) -> f32 {
        self.stamina / self.max_stamina
    }
}
