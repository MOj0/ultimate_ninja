use crate::animation_component::{AnimationComponent, AnimationState};
use crate::constants;
use crate::dead_component::DeadComponent;
use crate::entities;
use crate::entities::AABBCollisionComponent;
use crate::look_component::LookComponent;
use crate::move_component::MoveComponent;
use crate::transform_component::TransformComponent;
use crate::util;
use crate::Assets;
use crate::SpriteComponent;

use quad_rand as qrand;

pub struct Target {
    pub transform: TransformComponent,
    pub animation: AnimationComponent,
    pub dead_component: DeadComponent,
    pub move_component: MoveComponent,
    pub aabb: AABBCollisionComponent,
    pub look: LookComponent,

    pub target_dir: glam::Vec2,
    pub max_move_interval: f32,
    pub move_interval: f32,
}

impl Target {
    pub fn new(position: glam::Vec2, assets: &Assets, color: ggez::graphics::Color) -> Self {
        Self {
            transform: TransformComponent::new(
                position,
                constants::ENTITY_SIZE,
                util::compute_grid_index(&position),
            ),
            animation: util::build_walk_animation(
                &assets,
                util::compute_animation_duration(constants::TARGET_SPEED),
                color,
            ),
            dead_component: DeadComponent::new(
                SpriteComponent::new(assets.dead.clone(), ggez::graphics::Color::GREEN)
                    .scale(constants::SPRITE_SCALE),
                false,
            ),
            move_component: MoveComponent::new(constants::TARGET_SPEED),
            aabb: AABBCollisionComponent::new(ggez::graphics::Rect::new(
                position.x,
                position.y,
                constants::ENTITY_SIZE,
                constants::ENTITY_SIZE,
            )),
            look: LookComponent::new(
                glam::vec2(0., 1.),
                constants::GUARD_FOV,
                constants::GUARD_VIEW_DISTANCE,
                constants::N_FOV_RAYS,
            ),
            target_dir: glam::Vec2::ZERO,
            max_move_interval: 0.,
            move_interval: 0.,
        }
    }

    #[inline]
    pub fn set_angle(&mut self, dir: glam::Vec2) {
        if dir.length_squared() > 0. {
            self.transform.angle = util::get_vec_angle(dir);
        }
    }

    #[inline]
    pub fn set_colliding_vec_components(&mut self, colliding_axis: (bool, bool)) {
        self.aabb.colliding_axis = colliding_axis;
    }

    #[inline]
    pub fn is_dead(&self) -> bool {
        self.dead_component.is_dead
    }

    #[inline]
    pub fn set_dead(&mut self, is_dead: bool) {
        self.dead_component.is_dead = is_dead;
    }

    /// Since target can be dead, it has an additional sprite for dead state
    pub fn get_curr_animation_frame(&self) -> &SpriteComponent {
        if self.dead_component.is_dead {
            return &self.dead_component.sprite;
        }

        self.animation.get_curr_frame()
    }

    pub fn calculate_move_dir(&self) -> glam::Vec2 {
        let dx = 2. * constants::GUARD_FOV / self.look.ray_scales.len() as f32;

        let max_ray_scale = self
            .look
            .ray_scales
            .iter()
            .enumerate()
            .map(|(i, v)| (i, (*v * 100.) as i32))
            .max_by(|(_, a), (_, b)| a.cmp(b));

        if let Some((idx, max_ray)) = max_ray_scale {
            if max_ray < 60 {
                // Rotate 180 deg
                return util::vec_from_angle(self.transform.angle + constants::PI);
            }
            if max_ray < 100 {
                // Rotate 90 deg
                return util::vec_from_angle(
                    self.transform.angle + (qrand::gen_range(0., 1.)) * constants::PI / 2.,
                );
            }
            return util::vec_from_angle(
                self.transform.angle - constants::GUARD_FOV + idx as f32 * dx, // Go in the direction of the max_ray
            );
        }

        // Go in random direction
        return util::vec_from_angle(qrand::gen_range(0., 1.) * constants::PI * 2.);
    }

    pub fn update(&mut self, dt: f32) {
        if self.dead_component.is_dead {
            return;
        }

        if self.move_interval <= 0. {
            self.move_component
                .set_direction_normalized(self.target_dir);

            self.target_dir = self.calculate_move_dir();

            self.max_move_interval = qrand::gen_range(0.1, 0.4);
            self.move_interval = self.max_move_interval;
        } else {
            let lerped_dir = util::vec_lerp(
                self.move_component.direction,
                self.target_dir,
                self.max_move_interval - self.move_interval,
            );

            self.move_component.set_direction_normalized(lerped_dir);
        }

        self.look.look_at = self.move_component.direction;
        self.set_angle(self.move_component.direction);

        entities::move_entity(
            &mut self.transform,
            &self.move_component,
            self.aabb.colliding_axis,
        );

        self.aabb.rect.move_to(self.transform.position);

        self.animation.update(dt);

        if self.move_component.direction.length_squared() == 0. {
            self.animation.set_animation_state(AnimationState::Idle);
        } else {
            self.animation.set_animation_state(AnimationState::Active);
        }

        self.move_interval -= dt;
    }
}
