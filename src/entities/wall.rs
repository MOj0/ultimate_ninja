use crate::collision_component::AABBCollisionComponent;
use crate::constants;
use crate::entities;
use crate::move_component::MoveComponent;
use crate::sprite_component::SpriteComponent;
use crate::transform_component::TransformComponent;
use crate::GameState;

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

    pub fn get_colliding_axis(
        &self,
        transform: &TransformComponent,
        move_component: &MoveComponent,
        aabb: &AABBCollisionComponent,
    ) -> (bool, bool) {
        let rect_of_next_move = &entities::get_rect_of_next_move(transform, move_component, aabb);

        if self.aabb.check_collision(rect_of_next_move) {
            return self.aabb.get_colliding_axis(&aabb.rect);
        }

        (false, false)
    }
}

fn get_colliding_axis_all(
    walls: &Vec<Wall>,
    transform: &TransformComponent,
    move_component: &MoveComponent,
    aabb: &AABBCollisionComponent,
) -> (bool, bool) {
    walls.iter().fold((false, false), |init, wall| {
        let collding_axis = wall.get_colliding_axis(transform, move_component, aabb);
        (init.0 || collding_axis.0, init.1 || collding_axis.1)
    })
}

pub fn system(game_state: &mut GameState) {
    let player_colliding_axis = get_colliding_axis_all(
        &game_state.walls,
        &game_state.player.transform,
        &game_state.player.move_component,
        &game_state.player.aabb,
    );
    game_state.player.set_colliding_axis(player_colliding_axis);

    let target_colliding_axis = get_colliding_axis_all(
        &game_state.walls,
        &game_state.target.transform,
        &game_state.target.move_component,
        &game_state.target.aabb,
    );
    game_state.target.set_colliding_axis(target_colliding_axis);

    game_state.guards.iter_mut().for_each(|guard| {
        let guard_colliding_axis = get_colliding_axis_all(
            &game_state.walls,
            &guard.transform,
            &guard.move_component,
            &guard.aabb,
        );
        guard.set_colliding_axis(guard_colliding_axis);
    });
}
