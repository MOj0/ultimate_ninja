use crate::assets::Assets;
use crate::constants;
use crate::entities;
use crate::look_component::LookComponent;
use crate::move_component::MoveComponent;
use crate::sprite_component::SpriteComponent;
use crate::transform_component::TransformComponent;
use crate::util;
use crate::GameState;

pub struct Guard {
    pub transform: TransformComponent,
    pub sprite: SpriteComponent,
    pub move_component: MoveComponent,
    pub look: LookComponent,
    pub tmp_counter: f32,
}

impl Guard {
    pub fn new(
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::miniquad::GraphicsContext,
        position: glam::Vec2,
        assets: &Assets,
    ) -> Self {
        Self {
            transform: TransformComponent::new(position, constants::ENTITY_SIZE),
            sprite: SpriteComponent::new(assets.stand.clone(), ggez::graphics::Color::RED), // TODO: Optimize the image.clone()
            move_component: MoveComponent::new(constants::GUARD_SPEED),
            look: LookComponent::new(
                ctx,
                quad_ctx,
                glam::vec2(0., 1.),
                constants::GUARD_FOV,
                constants::GUARD_VIEW_DISTANCE,
            ),
            tmp_counter: 0.,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.move_component
            .set_direction_normalized(glam::vec2(self.tmp_counter.cos(), -self.tmp_counter.sin()));
        self.look.look_at = self.move_component.direction;

        entities::move_entity(&mut self.transform, &self.move_component);

        self.tmp_counter += dt;
    }
}

pub fn system(game_state: &mut GameState, ctx: &mut ggez::Context) {
    let dt = ggez::timer::delta(ctx).as_secs_f32();

    if is_player_detected(game_state) {
        game_state.player.is_detected = true;
    } else {
        game_state.player.is_detected = false;
    }

    game_state
        .guards
        .iter_mut()
        .for_each(|guard| guard.update(dt));
}

fn is_player_detected(game_state: &mut GameState) -> bool {
    let player_transform = &game_state.player.transform;

    game_state
        .guards
        .iter()
        .any(|guard| util::check_spotted(&guard.look, &guard.transform, &player_transform))
}
