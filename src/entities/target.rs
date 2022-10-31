use crate::assets::Assets;
use crate::entities;
use crate::move_component::MoveComponent;
use crate::sprite_component::SpriteComponent;
use crate::transform_component::TransformComponent;

pub struct Target {
    pub transform: TransformComponent,
    pub sprite: SpriteComponent,
    pub move_component: MoveComponent,

    pub is_dead: bool,
}

impl Target {
    pub fn new(position: glam::Vec2, size: f32, speed: f32, assets: &Assets) -> Self {
        Self {
            transform: TransformComponent::new(position, size),
            sprite: SpriteComponent::new(assets.stand.clone(), ggez::graphics::Color::GREEN), // TODO: Optimize image.clone
            move_component: MoveComponent::new(speed),
            is_dead: false,
        }
    }

    pub fn update_movement(&mut self) {
        self.move_component
            .set_direction_normalized(glam::vec2(-1., 0.));
        entities::move_entity(&mut self.transform, &self.move_component);
    }
}

// TODO: Target doesn't really have a system (yet)?
