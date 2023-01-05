use crate::constants;
use crate::entities::guards::Guard;
use crate::entities::guards::GuardState;
use crate::look_component::LookComponent;
use crate::transform_component::TransformComponent;
use crate::util;
use crate::Assets;

use quad_rand as qrand;

pub struct GuardHeavy {
    pub guard: Guard,

    pub is_using_next_look: bool,
}

impl GuardHeavy {
    pub fn new(
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::miniquad::GraphicsContext,
        position: glam::Vec2,
        assets: &Assets,
        color: ggez::graphics::Color,
    ) -> Self {
        let heavy_look_component = LookComponent::new_with_mesh(
            ctx,
            quad_ctx,
            glam::vec2(0., 1.),
            constants::GUARD_FOV,
            constants::GUARD_VIEW_DISTANCE_MEDIUM,
            constants::N_FOV_RAYS,
        );

        let mut guard = Guard::new(ctx, quad_ctx, position, assets, color);
        guard.add_look_component(heavy_look_component);

        Self {
            guard,

            is_using_next_look: false,
        }
    }

    #[inline]
    fn set_speed(&mut self, speed: f32) {
        self.guard.set_speed(speed);
    }

    pub fn is_dead(&self) -> bool {
        self.guard.is_dead()
    }

    pub fn update(
        &mut self,
        dt: f32,
        rect_objects: &Vec<(&ggez::graphics::Rect, isize)>,
        player_sound: &TransformComponent,
    ) {
        if self.guard.dead_component.is_dead {
            return;
        }

        match self.guard.guard_state {
            GuardState::HeardPlayer(dir_to_player) => self.guard.do_heard_player(dir_to_player),
            GuardState::Lookout(lookout_speed) => {
                let lookout_dir =
                    util::vec_from_angle(self.guard.transform.angle + dt * lookout_speed);

                self.guard.do_lookout(lookout_dir);

                if self.guard.move_interval <= 0. {
                    self.guard.guard_state = GuardState::Walk;
                    self.guard.next_look_component();
                }
            }
            GuardState::Walk => {
                if qrand::gen_range(1., 2000.) <= 1. {
                    self.guard.set_lookout(0.3, 0.4, 1., 2.);
                }

                self.guard.do_move(rect_objects, 2., 4.);
                self.set_speed(constants::GUARD_SPEED_MEDIUM);
            }
            GuardState::Alert => {
                if !self.is_using_next_look {
                    self.guard.next_look_component();
                    self.is_using_next_look = true;
                }

                self.guard.do_move(rect_objects, 2., 4.);
                self.set_speed(constants::GUARD_SPEED_FAST);
            }
        };

        self.guard.update(dt, player_sound);
    }
}
