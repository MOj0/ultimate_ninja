use ggez::{audio, GameResult};

pub struct SoundCollection {
    pub sounds: [audio::Source; 8],
    pub is_on: bool,
}

impl SoundCollection {
    pub fn play(&mut self, ctx: &mut ggez::Context, index: usize) -> GameResult<()> {
        if !self.is_on {
            return Ok(());
        }

        if let Some(source) = self.sounds.get_mut(index) {
            source.play(ctx).unwrap();
            return Ok(());
        }
        Err(ggez::error::GameError::SoundError)
    }
}
