use crate::constants;
use crate::entities::guards::Guard;
use crate::entities::guards::GuardState;
use crate::look_component::LookComponent;
use crate::transform_component::TransformComponent;
use crate::util;
use crate::Assets;

use quad_rand as qrand;

pub struct GuardScout {
    pub guard: Guard,
    pub scout_factor: f32,
}

impl GuardScout {
    pub fn new(
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::miniquad::GraphicsContext,
        position: glam::Vec2,
        assets: &Assets,
        color: ggez::graphics::Color,
        is_tutorial: bool,
    ) -> Self {
        let scout_look_component = LookComponent::new_with_mesh(
            ctx,
            quad_ctx,
            glam::vec2(0., 1.),
            constants::GUARD_FOV_SMALL,
            constants::GUARD_VIEW_DISTANCE_LONG,
            constants::N_FOV_RAYS,
        );

        let mut guard = Guard::new(ctx, quad_ctx, position, assets, color, is_tutorial);
        guard.add_look_component(scout_look_component);

        Self {
            guard,
            scout_factor: 1.,
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
                let lookout_dir = util::vec_from_angle(
                    self.guard.transform.angle + dt * lookout_speed * self.scout_factor,
                );
                self.guard.do_lookout(lookout_dir);

                if self.guard.move_interval <= 0. {
                    self.guard.look_components[0].update(&self.guard.transform, rect_objects);
                    self.guard.guard_state = GuardState::Walk;
                    self.guard.set_small_look_component();
                } else if self.guard.move_interval <= self.guard.max_move_interval / 2. {
                    self.scout_factor = -1.;
                }
            }
            GuardState::Walk => {
                if qrand::gen_range(1., 1000.) <= 12. || self.guard.is_tutorial {
                    self.guard.set_lookout(1.2, 1.6, 12., 16.);

                    self.scout_factor = 1.;
                    self.guard.set_large_look_component();
                    return;
                }

                self.guard.do_move(rect_objects, 5., 7.);
                self.set_speed(constants::GUARD_SPEED_SLOW);
            }
            GuardState::Alert => {
                self.guard.set_large_look_component();
                self.guard.do_move(rect_objects, 5., 7.);
                self.set_speed(constants::GUARD_SPEED);
            }
        };

        self.guard.update(dt, player_sound);
    }
}
