use crate::animation_component::AnimationState;
use crate::constants;
use crate::entities;
use crate::entities::guards;
use crate::entities::guards::Guard;
use crate::entities::guards::GuardState;
use crate::entities::wall::Wall;
use crate::look_component::LookComponent;
use crate::transform_component::TransformComponent;
use crate::util;
use crate::Assets;
use crate::Game;
use crate::GameState;

use quad_rand as qrand;

pub struct GuardHeavy {
    pub guard: Guard,

    pub move_dir: glam::Vec2,
    pub max_move_interval: f32,
    pub move_interval: f32,
    pub wall_move_interval: f32,
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

            move_dir: glam::Vec2::ZERO,
            max_move_interval: 0.,
            move_interval: 0.,
            wall_move_interval: 0.,
            is_using_next_look: false,
        }
    }

    #[inline]
    fn set_angle(&mut self, dir: glam::Vec2) {
        self.guard.set_angle(dir);
    }

    #[inline]
    fn set_speed(&mut self, speed: f32) {
        self.guard.set_speed(speed);
    }

    fn do_move(&mut self, rect_objects: &Vec<(&ggez::graphics::Rect, isize)>) {
        let min_ray_scale = self.guard.look_components[self.guard.look_idx]
            .ray_scales
            .iter()
            .min_by(|&&a, &&b| ((a * 100.) as i32).cmp(&((b * 100.) as i32)))
            .unwrap_or(&1.);

        let is_close_to_wall = *min_ray_scale < 0.6;

        if self.move_interval <= 0. || is_close_to_wall && self.wall_move_interval <= 0. {
            let new_dir = self
                .guard
                .compute_move_component
                .get_move_direction(&self.guard.transform, rect_objects)
                .normalize_or_zero();

            self.move_dir = new_dir;

            self.max_move_interval = qrand::gen_range(2., 4.);
            self.move_interval = self.max_move_interval;

            if is_close_to_wall {
                self.wall_move_interval = if self.guard.guard_state == GuardState::Walk {
                    0.75
                } else {
                    0.4
                };
            }
        } else {
            let lerped_dir = if self.max_move_interval != 0. {
                util::vec_lerp(
                    self.guard.move_component.direction,
                    self.move_dir,
                    1. - self.move_interval / self.max_move_interval,
                )
            } else {
                self.guard.move_component.direction
            };

            self.guard
                .move_component
                .set_direction_normalized(lerped_dir);
            self.set_angle(self.guard.move_component.direction);
        }
    }

    fn set_lookout(
        &mut self,
        speed_low: f32,
        speed_high: f32,
        duration_low: f32,
        duration_high: f32,
    ) {
        let lookout_speed = qrand::gen_range(speed_low, speed_high)
            * (qrand::gen_range(0., 1.) >= 0.5)
                .then_some(1.)
                .unwrap_or(-1.);

        self.guard.guard_state = GuardState::Lookout(lookout_speed);

        self.max_move_interval = qrand::gen_range(duration_low, duration_high);
        self.move_interval = self.max_move_interval;
    }

    pub fn update(
        &mut self,
        dt: f32,
        rect_objects: &Vec<(&ggez::graphics::Rect, isize)>,
        player_sound: &TransformComponent,
    ) {
        match self.guard.guard_state {
            GuardState::HeardPlayer(dir_to_player) => {
                self.set_speed(0.);

                let lerp_to_player = util::vec_lerp(
                    self.move_dir,
                    dir_to_player,
                    1. - self.move_interval / self.max_move_interval,
                );

                self.move_dir = lerp_to_player;
                self.guard.move_component.set_direction(lerp_to_player);

                self.set_angle(self.guard.move_component.direction);

                if self.move_interval <= 0. {
                    self.set_lookout(0.01, 0.7, 3., 4.);
                }
            }
            GuardState::Lookout(lookout_speed) => {
                let lookout_dir =
                    util::vec_from_angle(self.guard.transform.angle + dt * lookout_speed);

                self.set_speed(0.);
                self.move_dir = lookout_dir;

                self.guard.move_component.set_direction(lookout_dir);
                self.set_angle(self.guard.move_component.direction);

                if self.move_interval <= 0. {
                    self.guard.guard_state = GuardState::Walk;
                    self.guard.next_look_component();
                }
            }
            GuardState::Walk => {
                if qrand::gen_range(1., 2000.) <= 1. {
                    self.set_lookout(0.3, 0.4, 1., 2.);
                }

                self.do_move(rect_objects);
                self.set_speed(constants::GUARD_SPEED_MEDIUM);
            }
            GuardState::Alert => {
                if !self.is_using_next_look {
                    self.guard.next_look_component();
                    self.is_using_next_look = true;
                }

                self.do_move(rect_objects);
                self.set_speed(constants::GUARD_SPEED_FAST);
            }
        };

        entities::move_entity(
            &mut self.guard.transform,
            &self.guard.move_component,
            self.guard.aabb.colliding_axis,
        );

        self.guard.aabb.rect.move_to(self.guard.transform.position);

        self.guard.animation.update(dt);

        if self.guard.move_component.speed == 0. {
            self.guard
                .animation
                .set_animation_state(AnimationState::Idle);
        } else {
            self.guard
                .animation
                .set_animation_state(AnimationState::Active);
        }

        if !matches!(self.guard.guard_state, GuardState::HeardPlayer(_))
            && util::check_collision(&self.guard.transform, player_sound)
        {
            let dir_to_player =
                (player_sound.position - self.guard.transform.position).normalize_or_zero();
            self.guard.guard_state = GuardState::HeardPlayer(dir_to_player);

            self.max_move_interval = qrand::gen_range(0.3, 0.4);
            self.move_interval = self.max_move_interval;
        }

        self.move_interval = (self.move_interval - dt).max(-1.);
        self.wall_move_interval = (self.wall_move_interval - dt).max(-1.);
    }
}

pub fn system(ctx: &mut ggez::Context, game_state: &mut Game, dt: f32) {
    if is_transform_detected(
        &game_state.walls,
        &game_state.guards_heavy,
        &game_state.player.transform,
        game_state.player.is_stealth,
    ) {
        game_state.game_state = GameState::GameOver;
        game_state.sound_collection.play(ctx, 4).unwrap();
    }

    if !game_state.dead_target_detected
        && game_state.target.is_dead
        && is_transform_detected(
            &game_state.walls,
            &game_state.guards_heavy,
            &game_state.target.transform,
            false,
        )
    {
        guards::alert_all(ctx, game_state);
    }

    let aabb_objects = game_state
        .walls
        .iter()
        .map(|wall| (&wall.aabb.rect, wall.transform.grid_index))
        .collect::<Vec<(&ggez::graphics::Rect, isize)>>();

    let (p_position, p_sound_radius) = (
        game_state.player.transform.position,
        game_state.player.get_sound_radius(),
    );
    let player_sound = TransformComponent::new(p_position, p_sound_radius, -1);

    game_state
        .guards_heavy
        .iter_mut()
        .for_each(|guard| guard.update(dt, &aabb_objects, &player_sound));
}

fn is_transform_detected(
    walls: &Vec<Wall>,
    guards: &Vec<GuardHeavy>,
    transform: &TransformComponent,
    is_stealth: bool,
) -> bool {
    // If transform is stealth, it cannot be detected
    if is_stealth {
        return false;
    }

    let aabb_objects = walls
        .iter()
        .map(|wall| &wall.aabb.rect)
        .collect::<Vec<&ggez::graphics::Rect>>();

    guards.iter().any(|guard| {
        util::check_spotted(
            &guard.guard.look_components[guard.guard.look_idx],
            &guard.guard.transform,
            transform,
            &aabb_objects,
        )
    })
}
