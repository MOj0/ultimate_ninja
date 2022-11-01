use crate::animation_component::{AnimationComponent, AnimationState};
use crate::constants;
use crate::entities;
use crate::entities::AABBCollisionComponent;
use crate::look_component::LookComponent;
use crate::move_component::MoveComponent;
use crate::transform_component::TransformComponent;
use crate::util;
use crate::Assets;
use crate::GameState;

pub struct Guard {
    pub transform: TransformComponent,
    pub animation: AnimationComponent,
    pub move_component: MoveComponent,
    pub look: LookComponent,
    pub aabb: AABBCollisionComponent,

    pub tmp_rect_to_player: Option<ggez::graphics::Mesh>,
    pub tmp_counter: f32,
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
            transform: TransformComponent::new(position, constants::ENTITY_SIZE),
            animation: util::build_walk_animation(
                &assets,
                util::compute_animation_duration(constants::GUARD_SPEED),
                color,
            ),
            move_component: MoveComponent::new(constants::GUARD_SPEED),
            look: LookComponent::new(
                ctx,
                quad_ctx,
                glam::vec2(0., 1.),
                constants::GUARD_FOV,
                constants::GUARD_VIEW_DISTANCE,
            ),
            aabb: AABBCollisionComponent::new(ggez::graphics::Rect::new(
                position.x,
                position.y,
                constants::ENTITY_SIZE,
                constants::ENTITY_SIZE,
            )),
            tmp_rect_to_player: None,
            tmp_counter: 0.,
        }
    }

    #[inline]
    pub fn set_angle(&mut self, dir: glam::Vec2) {
        if dir.length_squared() > 0. {
            self.transform.angle = util::get_vec_angle(dir);
        }
    }

    #[inline]
    pub fn set_colliding_axis(&mut self, colliding_axis: (bool, bool)) {
        self.aabb.colliding_axis = colliding_axis;
    }

    pub fn update(&mut self, dt: f32) {
        self.move_component
            .set_direction_normalized(glam::vec2(self.tmp_counter.cos(), -self.tmp_counter.sin()));
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

        self.tmp_counter += dt;
    }
}

pub fn system(
    tmp_ctx: &mut ggez::Context,
    tmp_quad_ctx: &mut ggez::miniquad::Context,
    game_state: &mut GameState,
    dt: f32,
) {
    if is_player_detected(tmp_ctx, tmp_quad_ctx, game_state) {
        game_state.player.is_detected = true;
    } else {
        game_state.player.is_detected = false;
    }

    game_state
        .guards
        .iter_mut()
        .for_each(|guard| guard.update(dt));
}

fn is_player_detected(
    tmp_ctx: &mut ggez::Context,
    tmp_quad_ctx: &mut ggez::miniquad::Context,
    game_state: &mut GameState,
) -> bool {
    let player_transform = &game_state.player.transform;

    let aabb_objects = game_state
        .walls
        .iter()
        .map(|wall| &wall.aabb)
        .collect::<Vec<&AABBCollisionComponent>>();

    let dest = &game_state.player.transform;
    game_state.guards.iter_mut().for_each(|source| {
        let vec_to_dest = dest.position - source.transform.position;
        let rect_center = source.transform.position + vec_to_dest / 2.;
        let (rect_width, rect_height) = (
            (dest.position.x - source.transform.position.x).abs(),
            (dest.position.y - source.transform.position.y).abs(),
        );

        let mut rect_to_player =
            ggez::graphics::Rect::new(rect_center.x, rect_center.y, rect_width, rect_height);
        rect_to_player.translate(glam::vec2(-rect_width / 2., -rect_height / 2.));

        let m = ggez::graphics::Mesh::new_rectangle(
            tmp_ctx,
            tmp_quad_ctx,
            ggez::graphics::DrawMode::fill(),
            rect_to_player,
            ggez::graphics::Color::new(0.1, 0.2, 0.1, 0.2),
        )
        .unwrap();

        source.tmp_rect_to_player = Some(m);
    });

    game_state.guards.iter().any(|guard| {
        util::check_spotted(
            &guard.look,
            &guard.transform,
            &player_transform,
            &aabb_objects,
        )
    })
}
