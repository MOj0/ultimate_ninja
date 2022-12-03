use crate::constants;
use crate::transform_component::TransformComponent;
use crate::util;

use crate::Game;

pub struct CameraComponent {
    // TODO: Use static reference
    pub center: TransformComponent,
    pub size: glam::Vec2,
}

impl CameraComponent {
    #[inline]
    pub fn new(center: TransformComponent, size: glam::Vec2) -> Self {
        Self { center, size }
    }

    pub fn update(&mut self, center: TransformComponent) {
        self.center = center;

        self.center.position.x = util::clamp(
            self.center.position.x,
            self.size.x,
            constants::MAX_WORLD_X as f32 - self.size.x,
        );
        self.center.position.y = util::clamp(
            self.center.position.y,
            self.size.y,
            constants::MAX_WORLD_Y as f32 - self.size.y,
        );
    }

    #[inline]
    pub fn world_position(&self, global_pos: glam::Vec2) -> glam::Vec2 {
        global_pos - self.center.position + self.size
    }
}

pub fn system(game_state: &mut Game) {
    game_state
        .camera
        .update(game_state.player.transform.clone());
}
