use crate::animation_component::{AnimationComponent, AnimationState};
use crate::collision_component::AABBCollisionComponent;
use crate::move_component::MoveComponent;
use crate::transform_component::TransformComponent;
use crate::util;
use crate::GameState;

use crate::constants;
use crate::entities;

pub struct Player {
    pub transform: TransformComponent,
    pub animation: AnimationComponent,
    pub move_component: MoveComponent,
    pub aabb: AABBCollisionComponent,

    pub is_detected: bool,
}

impl Player {
    pub fn new(position: glam::Vec2, animation: AnimationComponent) -> Self {
        Self {
            transform: TransformComponent::new(position, constants::ENTITY_SIZE),
            animation,
            move_component: MoveComponent::new(constants::PLAYER_SPEED),
            aabb: AABBCollisionComponent::new(ggez::graphics::Rect::new(
                position.x,
                position.y,
                constants::ENTITY_SIZE,
                constants::ENTITY_SIZE,
            )),
            is_detected: true,
        }
    }

    #[inline]
    pub fn set_x_dir(&mut self, x_dir: f32) {
        self.move_component.set_x_dir(x_dir);
    }

    #[inline]
    pub fn set_y_dir(&mut self, y_dir: f32) {
        self.move_component.set_y_dir(y_dir);
    }

    #[inline]
    pub fn set_colliding_axis(&mut self, colliding_axis: (bool, bool)) {
        self.aabb.colliding_axis = colliding_axis;
    }

    pub fn update(&mut self, dt: f32) {
        entities::move_entity(
            &mut self.transform,
            &self.move_component,
            self.aabb.colliding_axis,
        );
        self.aabb.rect.move_to(self.transform.position);
        self.animation.update(dt);
    }
}

pub fn system(game_state: &mut GameState, dt: f32) {
    let player = &mut game_state.player;
    player.update(dt);

    if player.move_component.direction.length_squared() == 0. {
        player.animation.set_animation_state(AnimationState::Idle);
    } else {
        player.animation.set_animation_state(AnimationState::Active);
    }

    let target = &mut game_state.target;
    if util::check_collision(&player.transform, &target.transform) {
        target.is_dead = true;
    }
}
