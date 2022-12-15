use crate::util;

#[derive(Clone)]
pub struct TransformComponent {
    pub position: glam::Vec2,
    pub size: f32,
    pub angle: f32,
    pub grid_index: isize,
}

impl TransformComponent {
    #[inline]
    pub fn new(position: glam::Vec2, size: f32, grid_index: isize) -> Self {
        Self {
            position,
            size,
            angle: 0.,
            grid_index,
        }
    }

    #[inline]
    pub fn set(&mut self, dest: glam::Vec2) {
        self.position = dest;
    }

    #[inline]
    pub fn update(&mut self, dir: glam::Vec2) {
        self.position += dir;
        self.grid_index = util::compute_grid_index(&self.position);
    }
}
