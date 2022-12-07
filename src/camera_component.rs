use crate::constants;
use crate::transform_component::TransformComponent;
use crate::util;

use crate::Game;

pub struct CameraComponent {
    pub center: TransformComponent,
    pub size: glam::Vec2,
    pub lerp_delta: f32,
}

impl CameraComponent {
    #[inline]
    pub fn new(center: TransformComponent, size: glam::Vec2, lerp_delta: f32) -> Self {
        Self {
            center,
            size,
            lerp_delta,
        }
    }

    #[inline]
    pub fn set_lerp_delta(&mut self, lerp_delta: f32) {
        self.lerp_delta = lerp_delta;
    }

    pub fn update(&mut self, center: TransformComponent) {
        let mut target_center = center;
        target_center.position.x = util::clamp(
            target_center.position.x,
            self.size.x,
            constants::MAX_WORLD_X as f32 - self.size.x,
        );
        target_center.position.y = util::clamp(
            target_center.position.y,
            self.size.y,
            constants::MAX_WORLD_Y as f32 - self.size.y,
        );

        self.center.position = util::vec_lerp(
            self.center.position,
            target_center.position,
            self.lerp_delta,
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
