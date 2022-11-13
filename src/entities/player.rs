use crate::animation_component::{AnimationComponent, AnimationState};
use crate::collision_component::AABBCollisionComponent;
use crate::move_component::MoveComponent;
use crate::stamina_component::StaminaComponent;
use crate::transform_component::TransformComponent;
use crate::util;
use crate::Assets;
use crate::GameState;

use crate::constants;
use crate::entities;

pub struct Player {
    pub transform: TransformComponent,
    pub animation: AnimationComponent,
    pub move_component: MoveComponent,
    pub aabb: AABBCollisionComponent,
    pub stamina: StaminaComponent,

    pub is_detected: bool,
    pub stealth_intent: bool,
    pub is_stealth: bool,
}

impl Player {
    pub fn new(
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::miniquad::GraphicsContext,
        position: glam::Vec2,
        assets: &Assets,
        color: ggez::graphics::Color,
    ) -> Self {
        Self {
            transform: TransformComponent::new(position, constants::ENTITY_SIZE),
            animation: util::build_walk_animation(
                &assets,
                util::compute_animation_duration(constants::PLAYER_SPEED),
                color,
            ),
            move_component: MoveComponent::new(constants::PLAYER_SPEED),
            aabb: AABBCollisionComponent::new(ggez::graphics::Rect::new(
                position.x,
                position.y,
                constants::ENTITY_SIZE,
                constants::ENTITY_SIZE,
            )),
            is_detected: false,
            stamina: StaminaComponent::new(
                ctx,
                quad_ctx,
                100.,
                1.,
                ggez::graphics::Rect::new(10., 10., 200., 20.),
                ggez::graphics::Color::GREEN,
            ),
            stealth_intent: false,
            is_stealth: false,
        }
    }

    #[inline]
    pub fn set_stealth_intent(&mut self, stealth_intent: bool) {
        self.stealth_intent = stealth_intent;
        self.is_stealth = stealth_intent && self.stamina.stamina > 0.;
    }

    #[inline]
    pub fn set_angle(&mut self, dir: glam::Vec2) {
        if dir.length_squared() > 0. {
            self.transform.angle = util::get_vec_angle(dir);
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
    pub fn set_colliding_vec_components(&mut self, colliding_axis: (bool, bool)) {
        self.aabb.colliding_axis = colliding_axis;
    }

    pub fn update(&mut self, dt: f32) {
        self.stamina.update(self.stealth_intent);

        match self.is_stealth {
            false => self.animation.set_color(ggez::graphics::Color::BLACK),
            true => {
                self.animation
                    .set_color(ggez::graphics::Color::new(0., 0., 0., 0.25));

                // NOTE: If player is stealth, retrun
                return;
            }
        }

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

pub fn system(game_state: &mut GameState, dt: f32) {
    let player = &mut game_state.player;
    player.update(dt);

    let target = &mut game_state.target;
    if util::check_collision(&player.transform, &target.transform) {
        target.is_dead = true;
    }
}
