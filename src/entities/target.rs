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
    pub fn new(position: glam::Vec2, size: f32, speed: f32) -> Self {
        Self {
            transform: TransformComponent::new(position, size),
            sprite: SpriteComponent::new(),
            move_component: MoveComponent::new(speed),
            is_dead: false,
        }
    }

    pub fn update_movement(&mut self) {
        self.move_component
            .set_direction_normalized(glam::vec2(-1., 0.));
        entities::move_entity(&mut self.transform, &self.move_component);

        self.sprite.new_circle(
            ggez::graphics::DrawMode::fill(),
            self.transform.position,
            self.transform.size,
            0.25,
            ggez::graphics::Color::GREEN,
        );
    }
}

// TODO: Target doesn't really have a system (yet)?
