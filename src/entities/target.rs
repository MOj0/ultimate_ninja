use crate::animation_component::{AnimationComponent, AnimationState};
use crate::constants;
use crate::entities;
use crate::move_component::MoveComponent;
use crate::transform_component::TransformComponent;

pub struct Target {
    pub transform: TransformComponent,
    pub animation: AnimationComponent,
    pub move_component: MoveComponent,

    pub is_dead: bool,
}

impl Target {
    pub fn new(position: glam::Vec2, animation: AnimationComponent) -> Self {
        Self {
            transform: TransformComponent::new(position, constants::ENTITY_SIZE),
            animation,
            move_component: MoveComponent::new(constants::TARGET_SPEED),
            is_dead: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.move_component
            .set_direction_normalized(glam::vec2(-1., 0.));
        entities::move_entity(&mut self.transform, &self.move_component, (false, false)); // TODO: calculate collision instead of (false, false)

        self.animation.update(dt);
        if self.move_component.direction.length_squared() == 0. {
            self.animation.set_animation_state(AnimationState::Idle);
        } else {
            self.animation.set_animation_state(AnimationState::Active);
        }
    }
}

// TODO: Target doesn't really have a system (yet)?
