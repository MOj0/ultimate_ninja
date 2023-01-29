use crate::constants;
use crate::entities::guards::Guard;
use crate::entities::guards::GuardState;
use crate::transform_component::TransformComponent;
use crate::util;
use crate::Assets;

use quad_rand as qrand;

pub struct GuardBasic {
    pub guard: Guard,
}

impl GuardBasic {
    pub fn new(
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::miniquad::GraphicsContext,
        position: glam::Vec2,
        assets: &Assets,
        color: ggez::graphics::Color,
        is_tutorial: bool,
    ) -> Self {
        Self {
            guard: Guard::new(ctx, quad_ctx, position, assets, color, is_tutorial),
        }
    }

    pub fn is_dead(&self) -> bool {
        self.guard.is_dead()
    }

    pub fn update(
        &mut self,
        dt: f32,
        rect_objects: &Vec<(&ggez::graphics::Rect, isize)>,
        player_sound: &TransformComponent,
        guards_alerted: bool,
    ) {
        if self.is_dead() {
            return;
        }

        match self.guard.guard_state {
            GuardState::HeardPlayer(dir_to_player) => {
                self.guard.do_heard_player(dir_to_player, guards_alerted)
            }
            GuardState::Lookout(lookout_speed) => {
                let lookout_dir =
                    util::vec_from_angle(self.guard.transform.angle + dt * lookout_speed);
                self.guard.do_lookout(lookout_dir);

                if self.guard.move_interval <= 0. {
                    self.guard.guard_state = GuardState::Walk;
                }
            }
            GuardState::Walk => {
                if qrand::gen_range(1., 1000.) <= 5. || self.guard.is_tutorial {
                    self.guard.set_lookout(0.5, 0.9, 3., 5.);
                    return;
                }

                self.guard.do_move(rect_objects, 5., 7.);
                self.guard.set_speed(constants::GUARD_SPEED);
            }
            GuardState::Alert => {
                self.guard.do_move(rect_objects, 5., 7.);
                self.guard.set_speed(constants::GUARD_SPEED_MEDIUM);
            }
        };

        self.guard.update(dt, player_sound);
    }
}
