use crate::collision_component::AABBCollisionComponent;
use crate::constants;
use crate::util;

use crate::Game;

pub struct CameraComponent {
    pub center: glam::Vec2,
    pub size: glam::Vec2,
    pub lerp_delta: f32,
}

impl CameraComponent {
    #[inline]
    pub fn new(center: glam::Vec2, size: glam::Vec2, lerp_delta: f32) -> Self {
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

    #[inline]
    pub fn world_position(&self, global_pos: glam::Vec2) -> glam::Vec2 {
        global_pos - self.center + self.size
    }

    pub fn update(&mut self, center: glam::Vec2) {
        let mut target_center = center;
        target_center.x = util::clamp(
            target_center.x,
            self.size.x,
            constants::MAX_WORLD_X as f32 - self.size.x,
        );
        target_center.y = util::clamp(
            target_center.y,
            self.size.y,
            constants::MAX_WORLD_Y as f32 - self.size.y,
        );

        self.center = util::vec_lerp(self.center, target_center, self.lerp_delta);
    }

    pub fn contains(&self, object: &AABBCollisionComponent) -> bool {
        let camera_aabb = ggez::graphics::Rect::new(
            self.center.x - self.size.x,
            self.center.y - self.size.y,
            self.size.x * 2.,
            self.size.y * 2.,
        );

        camera_aabb.overlaps(&object.rect)
    }
}

pub fn system(game_state: &mut Game, dt: f32) {
    game_state
        .camera
        .update(game_state.player.transform.position);
}
