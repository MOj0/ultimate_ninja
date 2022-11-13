pub struct StaminaComponent {
    pub max_stamina: f32,
    pub stamina: f32,
    pub change_rate: f32,
    pub stamina_mesh: ggez::graphics::Mesh,
}

impl StaminaComponent {
    pub fn new(
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::miniquad::GraphicsContext,
        max_stamina: f32,
        change_rate: f32,
        rect: ggez::graphics::Rect,
        color: ggez::graphics::Color,
    ) -> Self {
        Self {
            max_stamina,
            stamina: max_stamina,
            change_rate,
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
        self.stamina = (self.stamina + self.change_rate * is_decreasing.then(|| -1.).unwrap_or(1.))
            .clamp(0., self.max_stamina);
    }

    #[inline]
    pub fn get_percentage(&self) -> f32 {
        self.stamina / self.max_stamina
    }
}
