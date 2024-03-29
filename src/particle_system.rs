use crate::camera_component::CameraComponent;
use crate::constants;
use crate::util;
use crate::Game;
use quad_rand as qrand;

pub struct ParticleSystem {
    emitters: Vec<ParticleEmitter>,
    pub is_activated: bool,
}

impl ParticleSystem {
    pub fn new(is_activated: bool) -> Self {
        Self {
            emitters: vec![],
            is_activated,
        }
    }

    #[inline]
    pub fn set_activated(&mut self, is_activated: bool) {
        self.is_activated = is_activated;
    }

    #[inline]
    pub fn add_emitter(&mut self, emitter: ParticleEmitter) {
        self.emitters.push(emitter);
    }

    pub fn update(&mut self, dt: f32) {
        self.emitters
            .iter_mut()
            .for_each(|emitter| emitter.update(dt));
    }

    pub fn reset(&mut self) {
        self.emitters.iter_mut().for_each(|emitter| emitter.reset());
    }

    pub fn draw(
        &mut self,
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::miniquad::Context,
        camera: &CameraComponent,
    ) -> ggez::GameResult {
        self.emitters
            .iter_mut()
            .map(|emitter| emitter.draw(ctx, quad_ctx, camera))
            .count();

        Ok(())
    }

    pub fn emit(&mut self, emitter_idx: usize, pos: glam::Vec2, n_particles: u32) {
        if !self.is_activated {
            return;
        }

        self.emitters[emitter_idx].set_pos(pos);
        self.emitters[emitter_idx].emit(n_particles);
    }

    pub fn set_scale(&mut self, emitter_idx: usize, scale: f32) {
        self.emitters[emitter_idx].set_scale(scale);
    }
}

pub struct ParticleEmitter {
    // Emitter
    emitter_position: glam::Vec2,
    max_particles: usize,
    min_lifetime: f32,
    max_lifetime: f32,
    scale_increase: f32,

    sprite_batch: ggez::graphics::spritebatch::SpriteBatch,

    // Particles
    color: ggez::graphics::Color, // All particles have the same color
    scale: f32,
    positions: Vec<glam::Vec2>,
    velocities: Vec<glam::Vec2>,
    scales: Vec<f32>,
    lifetimes: Vec<f32>,
}

impl ParticleEmitter {
    pub fn new(
        position: glam::Vec2,
        min_lifetime: f32,
        max_lifetime: f32,
        scale_increase: f32,
        color: ggez::graphics::Color,
        initial_scale: f32,
        max_particles: usize,
        particle_image: ggez::graphics::Image,
    ) -> Self {
        let sprite_batch = ggez::graphics::spritebatch::SpriteBatch::new(particle_image);

        Self {
            emitter_position: position,
            max_particles,
            min_lifetime,
            max_lifetime,
            scale_increase,
            sprite_batch,
            color,
            scale: initial_scale,
            positions: vec![glam::Vec2::ZERO; max_particles],
            velocities: vec![glam::Vec2::ZERO; max_particles],
            scales: vec![0.; max_particles],
            lifetimes: vec![0.; max_particles],
        }
    }

    pub fn reset(&mut self) {
        self.positions = vec![glam::Vec2::ZERO; self.max_particles];
        self.velocities = vec![glam::Vec2::ZERO; self.max_particles];
        self.scales = vec![0.; self.max_particles];
        self.lifetimes = vec![0.; self.max_particles];
    }

    #[inline]
    pub fn set_pos(&mut self, pos: glam::Vec2) {
        self.emitter_position = pos;
    }

    #[inline]
    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn emit(&mut self, n_particles: u32) {
        let mut created_particles = 0;

        for (i, lifetime) in self.lifetimes.iter_mut().enumerate() {
            if *lifetime <= 0. {
                // Create new particle
                self.positions[i] = self.emitter_position;
                self.velocities[i] = util::vec_from_angle(qrand::gen_range(0., 2. * constants::PI));
                self.scales[i] = self.scale;
                *lifetime = qrand::gen_range(self.min_lifetime, self.max_lifetime);

                created_particles += 1;
                if created_particles >= n_particles {
                    return;
                }
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.lifetimes
            .iter_mut()
            .enumerate()
            .filter(|(_, lifetime)| **lifetime > 0.)
            .for_each(|(particle_idx, lifetime)| {
                self.positions[particle_idx] += self.velocities[particle_idx];
                self.scales[particle_idx] += self.scale_increase;
                *lifetime -= dt;
            });
    }

    pub fn draw(
        &mut self,
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::miniquad::Context,
        camera: &CameraComponent,
    ) -> ggez::GameResult {
        self.sprite_batch.clear();
        for (particle_idx, _) in self
            .lifetimes
            .iter()
            .enumerate()
            .filter(|(_, lifetime)| **lifetime > 0.)
        {
            let pos = camera.world_position(self.positions[particle_idx]);
            let scale = self.scales[particle_idx];

            let draw_param = ggez::graphics::DrawParam::default()
                .dest(pos)
                .offset(glam::Vec2::splat(0.5))
                .scale(glam::Vec2::splat(scale))
                .color(self.color);

            self.sprite_batch.add(draw_param);
        }

        return ggez::graphics::draw(
            ctx,
            quad_ctx,
            &self.sprite_batch,
            ggez::graphics::DrawParam::default(),
        );
    }
}

pub fn system(game_state: &mut Game, dt: f32) {
    game_state.particle_system.update(dt);
}
