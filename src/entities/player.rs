use crate::assets::Assets;
use crate::move_component::MoveComponent;
use crate::sprite_component::SpriteComponent;
use crate::transform_component::TransformComponent;
use crate::util;
use crate::GameState;

use crate::entities;

pub struct Player {
    pub transform: TransformComponent,
    pub sprite: SpriteComponent,
    pub move_component: MoveComponent,
    pub is_detected: bool,
}

impl Player {
    pub fn new(position: glam::Vec2, size: f32, speed: f32, assets: &Assets) -> Self {
        Self {
            transform: TransformComponent::new(position, size),
            sprite: SpriteComponent::new(assets.stand.clone(), ggez::graphics::Color::BLACK), // TODO: Optimize image.clone()
            move_component: MoveComponent::new(speed),
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
    pub fn update(&mut self) {
        entities::move_entity(&mut self.transform, &self.move_component);

        let player_color = if self.is_detected {
            ggez::graphics::Color::BLUE
        } else {
            ggez::graphics::Color::BLACK
        };
        self.sprite.set_color(player_color);
    }
}

pub fn system(game_state: &mut GameState) {
    let player = &mut game_state.player;
    player.update();

    let target = &mut game_state.target;
    if util::check_collision(&player.transform, &target.transform) {
        target.is_dead = true;
    }
}
