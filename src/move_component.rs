pub struct MoveComponent {
    pub direction: glam::Vec2,
    pub speed: f32,
}

impl MoveComponent {
    pub fn new(speed: f32) -> Self {
        Self {
            direction: glam::Vec2::default(),
            speed,
        }
    }

    #[inline]
    pub fn set_direction(&mut self, direction: glam::Vec2) {
        self.direction = direction
    }

    pub fn set_direction_normalized(&mut self, direction: glam::Vec2) {
        match direction.try_normalize() {
            None => self.direction = direction,
            Some(dir_normalized) => self.direction = dir_normalized,
        }
    }

    #[inline]
    pub fn set_x_dir(&mut self, x_dir: f32) {
        self.set_direction(glam::vec2(x_dir, self.direction.y));
    }
    #[inline]
    pub fn set_y_dir(&mut self, y_dir: f32) {
        self.set_direction(glam::vec2(self.direction.x, y_dir));
    }

    #[inline]
    pub fn set_x_dir_normalized(&mut self, x_dir: f32) {
        self.set_direction_normalized(glam::vec2(x_dir, self.direction.y));
    }
    #[inline]
    pub fn set_y_dir_normalized(&mut self, y_dir: f32) {
        self.set_direction_normalized(glam::vec2(self.direction.x, y_dir));
    }
}
