use crate::constants;
use crate::sprite_component::SpriteComponent;
use crate::transform_component::TransformComponent;
use crate::GameState;

pub struct Exit {
    pub transform: TransformComponent,
    pub sprite: SpriteComponent,
    pub scale_rotation_counter: f32,
    pub player_exited: bool,
}

impl Exit {
    pub fn new(position: glam::Vec2, sprite: SpriteComponent) -> Self {
        Self {
            transform: TransformComponent::new(position, constants::ENTITY_SIZE), // TODO: Define a more appropriate scale for collision checking
            sprite,
            scale_rotation_counter: 0.,
            player_exited: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.sprite
            .set_scale(self.scale_rotation_counter.sin() * 0.05 + 0.4);

        self.scale_rotation_counter = (self.scale_rotation_counter + dt) % (2. * constants::PI);
    }
}

pub fn system(game_state: &mut GameState, dt: f32) {
    game_state.exit.update(dt);
}
