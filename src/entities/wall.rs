use crate::collision_component::AABBCollisionComponent;
use crate::constants;
use crate::entities;
use crate::move_component::MoveComponent;
use crate::sprite_component::SpriteComponent;
use crate::transform_component::TransformComponent;
use crate::Game;

pub struct Wall {
    pub transform: TransformComponent,
    pub aabb: AABBCollisionComponent,
    pub sprite: SpriteComponent,
}

impl Wall {
    pub fn new(position: glam::Vec2, width: f32, height: f32, sprite: SpriteComponent) -> Self {
        Self {
            transform: TransformComponent::new(position, width * height),
            aabb: AABBCollisionComponent::new(ggez::graphics::Rect::new(
                position.x, position.y, width, height,
            )),
            sprite: sprite.scale(glam::vec2(
                width / constants::SPRITE_SIZE as f32,
                height / constants::SPRITE_SIZE as f32,
            )),
        }
    }

    pub fn get_colliding_vec_components(
        &self,
        aabb: &AABBCollisionComponent,
        rect_of_next_move: &ggez::graphics::Rect,
    ) -> (bool, bool) {
        if self.aabb.check_collision(rect_of_next_move) {
            return self.aabb.get_colliding_axis(&aabb.rect);
        }

        (false, false)
    }
}

fn get_colliding_vec_components_all(
    walls: &Vec<Wall>,
    transform: &TransformComponent,
    move_component: &MoveComponent,
    aabb: &AABBCollisionComponent,
) -> (bool, bool) {
    let rect_of_next_move = &entities::get_rect_of_next_move(transform, move_component, aabb);

    walls.iter().fold((false, false), |init, wall| {
        let collding_vec_components = wall.get_colliding_vec_components(aabb, rect_of_next_move);
        (
            init.0 || collding_vec_components.0,
            init.1 || collding_vec_components.1,
        )
    })
}

pub fn check_collision(game_state: &mut Game) {
    let player_colliding_vec_components = get_colliding_vec_components_all(
        &game_state.walls,
        &game_state.player.transform,
        &game_state.player.move_component,
        &game_state.player.aabb,
    );
    game_state
        .player
        .set_colliding_vec_components(player_colliding_vec_components);

    let target_colliding_vec_components = get_colliding_vec_components_all(
        &game_state.walls,
        &game_state.target.transform,
        &game_state.target.move_component,
        &game_state.target.aabb,
    );
    game_state
        .target
        .set_colliding_vec_components(target_colliding_vec_components);

    game_state.guards.iter_mut().for_each(|guard| {
        let guard_colliding_axis = get_colliding_vec_components_all(
            &game_state.walls,
            &guard.transform,
            &guard.move_component,
            &guard.aabb,
        );
        guard.set_colliding_vec_components(guard_colliding_axis);
    });
}
