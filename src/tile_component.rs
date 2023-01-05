use crate::constants;
use crate::util;
use crate::SpriteComponent;
use crate::TransformComponent;

pub struct TileComponent {
    pub transform: TransformComponent,
    pub sprite: SpriteComponent,
    pub brightness: f32,
}

impl TileComponent {
    pub fn new(position: glam::Vec2, width: f32, height: f32, sprite: SpriteComponent) -> Self {
        Self {
            transform: TransformComponent::new(
                position,
                width * height,
                util::compute_grid_index(&position),
            ),
            sprite: sprite.scale(glam::vec2(
                width / constants::SPRITE_SIZE as f32,
                height / constants::SPRITE_SIZE as f32,
            )),
            brightness: 0.6,
        }
    }
}
