use crate::animation_component::{AnimationComponent, AnimationState};
use crate::constants;
use crate::entities;
use crate::entities::AABBCollisionComponent;
use crate::move_component::MoveComponent;
use crate::transform_component::TransformComponent;
use crate::util;
use crate::Assets;
use crate::SpriteComponent;

pub struct Target {
    pub transform: TransformComponent,
    pub animation: AnimationComponent,
    pub sprite_dead: SpriteComponent,
    pub move_component: MoveComponent,
    pub aabb: AABBCollisionComponent,

    pub is_dead: bool,
}

impl Target {
    pub fn new(position: glam::Vec2, assets: &Assets, color: ggez::graphics::Color) -> Self {
        Self {
            transform: TransformComponent::new(position, constants::ENTITY_SIZE),
            animation: util::build_walk_animation(
                &assets,
                util::compute_animation_duration(constants::TARGET_SPEED),
                color,
            ),
            sprite_dead: SpriteComponent::new(assets.dead.clone(), ggez::graphics::Color::GREEN)
                .scale(constants::SPRITE_SCALE),
            move_component: MoveComponent::new(constants::TARGET_SPEED),
            aabb: AABBCollisionComponent::new(ggez::graphics::Rect::new(
                position.x,
                position.y,
                constants::ENTITY_SIZE,
                constants::ENTITY_SIZE,
            )),
            is_dead: false,
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

    /// Since target can be dead, it has an additional sprite for dead state
    pub fn get_curr_animation_frame(&self) -> &SpriteComponent {
        if self.is_dead {
            return &self.sprite_dead;
        }

        self.animation.get_curr_frame()
    }

    pub fn update(&mut self, dt: f32) {
        if self.is_dead {
            return;
        }

        self.move_component
            .set_direction_normalized(glam::vec2(-1., 0.));
        entities::move_entity(
            &mut self.transform,
            &self.move_component,
            self.aabb.colliding_axis,
        );
        self.set_angle(self.move_component.direction);

        self.aabb.rect.move_to(self.transform.position);

        self.animation.update(dt);
        if self.move_component.direction.length_squared() == 0. {
            self.animation.set_animation_state(AnimationState::Idle);
        } else {
            self.animation.set_animation_state(AnimationState::Active);
        }
    }
}

// TODO: Target doesn't really have a system (yet)?
