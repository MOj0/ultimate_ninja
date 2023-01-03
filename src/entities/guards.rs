pub mod guard_basic;
pub mod guard_scout;

use crate::animation_component::AnimationComponent;
use crate::compute_move_component::ComputeMoveComponent;
use crate::constants;
use crate::entities::AABBCollisionComponent;
use crate::look_component::LookComponent;
use crate::move_component::MoveComponent;
use crate::transform_component::TransformComponent;
use crate::util;
use crate::Assets;
use crate::Game;

pub struct Guard {
    pub guard_state: GuardState,
    pub transform: TransformComponent,
    pub animation: AnimationComponent,
    pub move_component: MoveComponent,
    pub aabb: AABBCollisionComponent,
    pub compute_move_component: ComputeMoveComponent,
    pub look_components: Vec<LookComponent>,
    pub look_idx: usize,

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
            transform: TransformComponent::new(
                position,
                constants::ENTITY_SIZE,
                util::compute_grid_index(&position),
            ),
            animation: util::build_walk_animation(
                &assets,
                util::compute_animation_duration(constants::GUARD_SPEED),
                color,
            ),
            move_component: MoveComponent::new(constants::GUARD_SPEED),
            aabb: AABBCollisionComponent::new(ggez::graphics::Rect::new(
                position.x,
                position.y,
                constants::ENTITY_SIZE,
                constants::ENTITY_SIZE,
            )),
            compute_move_component: ComputeMoveComponent::new(
                ctx,
                quad_ctx,
                8,
                constants::GUARD_VIEW_DISTANCE,
            ),
            look_components: vec![LookComponent::new_with_mesh(
                ctx,
                quad_ctx,
                glam::vec2(0., 1.),
                constants::GUARD_FOV,
                constants::GUARD_VIEW_DISTANCE,
                constants::N_FOV_RAYS,
            )],
            look_idx: 0,
            look_color: ggez::graphics::Color::WHITE,
        }
    }

    #[inline]
    fn set_angle(&mut self, dir: glam::Vec2) {
        if dir.length_squared() > 0. {
            self.transform.angle = util::get_vec_angle(dir);
        }

        self.look_components[self.look_idx].look_at = dir;
    }

    #[inline]
    fn add_look_component(&mut self, look_component: LookComponent) {
        self.look_components.push(look_component);
    }

    #[inline]
    fn next_look_component(&mut self) {
        self.look_idx = (self.look_idx + 1) % self.look_components.len();
    }

    #[inline]
    fn set_speed(&mut self, speed: f32) {
        self.move_component.speed = speed;
    }

    #[inline]
    fn set_look_color(&mut self, look_color: ggez::graphics::Color) {
        self.look_color = look_color;
    }

    #[inline]
    pub fn set_colliding_vec_components(&mut self, colliding_axis: (bool, bool)) {
        self.aabb.colliding_axis = colliding_axis;
    }
}

pub fn alert_all(ctx: &mut ggez::Context, game_state: &mut Game) {
    game_state
        .get_all_guards_mut()
        .iter_mut()
        .for_each(|guard| {
            guard.guard_state = GuardState::Alert;
            guard.set_look_color(ggez::graphics::Color::RED);
        });

    game_state.dead_target_detected = true;
    game_state.sound_collection.play(ctx, 6).unwrap();
}
