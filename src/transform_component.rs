pub struct TransformComponent {
    pub position: glam::Vec2,
    pub size: f32,
}

impl TransformComponent {
    #[inline]
    pub fn new(position: glam::Vec2, size: f32) -> Self {
        Self { position, size }
    }

    #[inline]
    pub fn set(&mut self, dest: glam::Vec2) {
        self.position = dest;
    }

    #[inline]
    pub fn update(&mut self, dir: glam::Vec2) {
        self.position += dir;
    }
}
