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
}

impl GuardHeavy {
    pub fn new(
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::miniquad::GraphicsContext,
        position: glam::Vec2,
        assets: &Assets,
        color: ggez::graphics::Color,
        is_tutorial: bool,
    ) -> Self {
        let heavy_look_component = LookComponent::new_with_mesh(
            ctx,
            quad_ctx,
            glam::vec2(0., 1.),
            constants::GUARD_FOV,
            constants::GUARD_VIEW_DISTANCE_MEDIUM,
            constants::N_FOV_RAYS,
        );

        let mut guard = Guard::new(ctx, quad_ctx, position, assets, color, is_tutorial);
        guard.add_look_component(heavy_look_component);

        Self { guard }
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
        guards_alerted: bool,
    ) {
        if self.guard.dead_component.is_dead {
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
                    self.guard.set_small_look_component();
                }
            }
            GuardState::Walk => {
                if qrand::gen_range(1., 1000.) <= 5. || self.guard.is_tutorial {
                    self.guard.set_lookout(0.3, 0.4, 4., 6.);
                    self.guard.set_large_look_component();
                    return;
                }

                self.guard.do_move(rect_objects, 5., 7.);
                self.set_speed(constants::GUARD_SPEED_MEDIUM);
            }
            GuardState::Alert => {
                self.guard.set_large_look_component();
                self.guard.do_move(rect_objects, 5., 7.);
                self.set_speed(constants::GUARD_SPEED_FAST);
            }
        };

        self.guard.update(dt, player_sound);
    }
}
