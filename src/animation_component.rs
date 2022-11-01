use crate::sprite_component::SpriteComponent;

pub struct AnimationComponent {
    pub animation: Vec<SpriteComponent>,
    pub index: usize,
    pub duration: f32,
    pub duration_counter: f32,
    pub animation_state: AnimationState,
}

impl AnimationComponent {
    pub fn new(animation: Vec<SpriteComponent>, duration: f32) -> Self {
        Self {
            animation,
            index: 0,
            duration,
            duration_counter: 0.,
            animation_state: AnimationState::Idle,
        }
    }

    pub fn set_color(&mut self, color: ggez::graphics::Color) {
        self.animation
            .iter_mut()
            .for_each(|sprite| sprite.set_color(color));
    }

    pub fn set_animation_state(&mut self, state: AnimationState) {
        if state == AnimationState::Idle {
            self.index = 0;
        }

        self.animation_state = state;
    }

    pub fn get_curr_frame(&self) -> &SpriteComponent {
        self.animation
            .get(self.index)
            .expect("index is out of range")
    }

    pub fn update(&mut self, dt: f32) {
        if self.animation_state == AnimationState::Idle {
            return;
        }

        if self.duration_counter >= self.duration {
            self.index = (self.index + 1) % self.animation.len();
            self.duration_counter = 0.;
        }

        self.duration_counter += dt;
    }
}

#[derive(PartialEq)]
pub enum AnimationState {
    Idle,
    Active,
}
