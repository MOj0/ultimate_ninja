use crate::collision_component::AABBCollisionComponent;
use crate::constants;
use crate::entities::player::Player;
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

    pub fn get_colliding_components(&self, player: &Player) -> (bool, bool) {
        if self.aabb.check_collision(&player.get_rect_of_next_move()) {
            return self.aabb.get_colliding_components(&player.aabb.rect);
        }
        (false, false)
    }
}

pub fn system(game_state: &mut GameState) {
    let player_colliding_components = game_state.walls.iter().fold((false, false), |init, wall| {
        let collding_components = wall.get_colliding_components(&game_state.player);
        (
            init.0 || collding_components.0,
            init.1 || collding_components.1,
        )
    });

    game_state
        .player
        .set_colliding_components(player_colliding_components);
}
