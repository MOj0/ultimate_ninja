use crate::sprite_component::SpriteComponent;
use crate::transform_component::TransformComponent;

pub struct TeleportComponent {
    pub location: Option<TransformComponent>,
    pub sprite: SpriteComponent,
}

impl TeleportComponent {
    pub fn new(sprite: SpriteComponent) -> Self {
        Self {
            location: None,
            sprite,
        }
    }

    #[inline]
    pub fn set_location(&mut self, location: TransformComponent) {
        self.location = Some(location);
    }
}
