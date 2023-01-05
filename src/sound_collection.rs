use ggez::{audio, GameResult};

pub struct SoundCollection {
    pub sounds: [audio::Source; 9],
    pub is_on: bool,
}

impl SoundCollection {
    pub fn play(&mut self, ctx: &mut ggez::Context, index: usize) -> GameResult<()> {
        if !self.is_on {
            return Ok(());
        }

        if let Some(source) = self.sounds.get_mut(index) {
            return source.play(ctx);
        }

        Err(ggez::error::GameError::SoundError)
    }

    pub fn set_volume_to(
        &mut self,
        ctx: &mut ggez::Context,
        index: usize,
        volume: f32,
    ) -> GameResult<()> {
        self.sounds[index].set_volume(ctx, volume)
    }
}
