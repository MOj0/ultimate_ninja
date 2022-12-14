use crate::animation_component::{AnimationComponent, AnimationState};
use crate::constants;
use crate::entities;
use crate::entities::wall::Wall;
use crate::entities::AABBCollisionComponent;
use crate::look_component::LookComponent;
use crate::move_component::MoveComponent;
use crate::transform_component::TransformComponent;
use crate::util;
use crate::Assets;
use crate::Game;
use crate::GameState;

use quad_rand as qrand;

pub struct Guard {
    pub guard_state: GuardState,
    pub transform: TransformComponent,
    pub animation: AnimationComponent,
    pub move_component: MoveComponent,
    pub look: LookComponent,
    pub aabb: AABBCollisionComponent,

    pub move_dir: glam::Vec2,
    pub max_move_interval: f32,
    pub move_interval: f32,
    pub look_color: ggez::graphics::Color,
}

#[derive(PartialEq)]
pub enum GuardState {
    Lookout(f32),
    Walk,
    Alert,
}

impl Guard {
    pub fn new(
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::miniquad::GraphicsContext,
        position: glam::Vec2,
        assets: &Assets,
        color: ggez::graphics::Color,
    ) -> Self {
        Self {
            guard_state: GuardState::Walk,
            transform: TransformComponent::new(position, constants::ENTITY_SIZE),
            animation: util::build_walk_animation(
                &assets,
                util::compute_animation_duration(constants::GUARD_SPEED),
                color,
            ),
            move_component: MoveComponent::new(constants::GUARD_SPEED),
            look: LookComponent::new_with_mesh(
                ctx,
                quad_ctx,
                glam::vec2(0., 1.),
                constants::GUARD_FOV,
                constants::GUARD_VIEW_DISTANCE,
                constants::N_FOV_RAYS,
            ),
            aabb: AABBCollisionComponent::new(ggez::graphics::Rect::new(
                position.x,
                position.y,
                constants::ENTITY_SIZE,
                constants::ENTITY_SIZE,
            )),
            move_dir: glam::Vec2::ZERO,
            max_move_interval: 0.,
            move_interval: 0.,
            look_color: ggez::graphics::Color::WHITE,
        }
    }

    #[inline]
    pub fn set_angle(&mut self, dir: glam::Vec2) {
        if dir.length_squared() > 0. {
            self.transform.angle = util::get_vec_angle(dir);
        }
    }

    #[inline]
    pub fn set_speed(&mut self, speed: f32) {
        self.move_component.speed = speed;
    }

    #[inline]
    pub fn set_look_color(&mut self, look_color: ggez::graphics::Color) {
        self.look_color = look_color;
    }

    #[inline]
    pub fn set_colliding_vec_components(&mut self, colliding_axis: (bool, bool)) {
        self.aabb.colliding_axis = colliding_axis;
    }

    pub fn calculate_move_dir(&self) -> glam::Vec2 {
        let dx = 2. * constants::GUARD_FOV / self.look.ray_scales.len() as f32;

        let (idx, max_ray) = self
            .look
            .ray_scales
            .iter()
            .enumerate()
            .map(|(i, v)| (i, (*v * 100.) as i32))
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap_or_default();

        if max_ray < 60 {
            // Rotate 180 deg
            return util::vec_from_angle(self.transform.angle + constants::PI);
        }
        if max_ray < 100 {
            // Rotate 90 deg
            return util::vec_from_angle(
                self.transform.angle + (qrand::gen_range::<f32>(0., 1.)) * constants::PI / 2.,
            );
        }

        if self.guard_state == GuardState::Alert {
            return util::vec_from_angle(
                self.transform.angle - constants::GUARD_FOV + idx as f32 * dx, // Go in the direction of the max_ray
            );
        }

        // GuardState == Walk
        util::vec_from_angle(
            self.transform.angle + qrand::gen_range::<f32>(-1., 1.) * constants::PI / 3.,
        )
    }

    pub fn update(&mut self, dt: f32) {
        if self.move_interval <= 0. {
            if self.guard_state == GuardState::Walk && qrand::gen_range(1., 100.) <= 3. {
                let lookout_speed = qrand::gen_range(0.5, 0.9)
                    * (qrand::gen_range(-2, 1) >= 0).then_some(1.).unwrap_or(-1.);

                self.guard_state = GuardState::Lookout(lookout_speed);
                self.move_interval = qrand::gen_range(3., 5.);
            } else {
                if self.guard_state != GuardState::Alert {
                    self.guard_state = GuardState::Walk;
                }

                self.move_component.set_direction_normalized(self.move_dir);

                self.move_dir = self.calculate_move_dir();

                self.max_move_interval = qrand::gen_range::<f32>(0.2, 0.4);
                self.move_interval = self.max_move_interval;
            }
        } else {
            let lerped_dir = util::vec_lerp(
                self.move_component.direction,
                self.move_dir,
                self.max_move_interval - self.move_interval,
            );

            self.move_component.set_direction_normalized(lerped_dir);
        }

        match self.guard_state {
            GuardState::Lookout(lookout_speed) => {
                let lookout_dir = util::vec_from_angle(self.transform.angle + dt * lookout_speed);

                self.set_speed(0.);
                self.move_dir = lookout_dir;
                self.move_component.set_direction(lookout_dir);
            }
            GuardState::Walk => self.set_speed(constants::GUARD_SPEED),
            GuardState::Alert => self.set_speed(constants::GUARD_SPEED_FAST),
        };

        self.look.look_at = self.move_component.direction;
        self.set_angle(self.move_component.direction);

        entities::move_entity(
            &mut self.transform,
            &self.move_component,
            self.aabb.colliding_axis,
        );

        self.aabb.rect.move_to(self.transform.position);

        self.animation.update(dt);

        if self.move_component.speed == 0. {
            self.animation.set_animation_state(AnimationState::Idle);
        } else {
            self.animation.set_animation_state(AnimationState::Active);
        }

        self.move_interval -= dt;
    }
}

pub fn system(ctx: &mut ggez::Context, game_state: &mut Game, dt: f32) {
    if is_transform_detected(
        &game_state.walls,
        &game_state.guards,
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
            &game_state.guards,
            &game_state.target.transform,
            false,
        )
    {
        game_state.guards.iter_mut().for_each(|guard| {
            guard.guard_state = GuardState::Alert;
            guard.set_look_color(ggez::graphics::Color::RED);
        });

        game_state.dead_target_detected = true;
        game_state.sound_collection.play(ctx, 6).unwrap();
    }

    game_state
        .guards
        .iter_mut()
        .for_each(|guard| guard.update(dt));
}

fn is_transform_detected(
    walls: &Vec<Wall>,
    guards: &Vec<Guard>,
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

    guards
        .iter()
        .any(|guard| util::check_spotted(&guard.look, &guard.transform, transform, &aabb_objects))
}
