use crate::SpriteComponent;

pub struct DeadComponent {
    pub sprite: SpriteComponent,
    pub is_dead: bool,
}

impl DeadComponent {
    pub fn new(sprite: SpriteComponent, is_dead: bool) -> Self {
        Self { sprite, is_dead }
    }
}
