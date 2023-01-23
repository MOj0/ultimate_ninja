use crate::camera_component::CameraComponent;
use crate::sprite_component;
use crate::sprite_component::SpriteComponent;
use crate::util;
use crate::Game;

pub struct OverlaySystem {
    overlay_items: Vec<OverlayItem>,
    active_indices: Vec<bool>,
    counter: f32,
}

impl OverlaySystem {
    pub fn new() -> Self {
        Self {
            overlay_items: vec![],
            active_indices: vec![],
            counter: 0.,
        }
    }

    #[inline]
    pub fn add_item(&mut self, overlay_item: OverlayItem) {
        self.overlay_items.push(overlay_item);
        self.active_indices.push(false);
    }

    #[inline]
    pub fn set_active_at(&mut self, idx: usize, active: bool) {
        self.active_indices[idx] = active;
    }

    #[inline]
    pub fn set_pos_at(&mut self, idx: usize, position: glam::Vec2) {
        self.overlay_items
            .get_mut(idx)
            .unwrap()
            .set_position(position);
    }

    #[inline]
    pub fn set_rot_at(&mut self, idx: usize, rotation: f32) {
        self.overlay_items
            .get_mut(idx)
            .unwrap()
            .set_rotation(rotation);
    }

    pub fn update(&mut self, dt: f32) {
        self.overlay_items
            .iter_mut()
            .enumerate()
            .filter(|(i, _)| self.active_indices[*i])
            .for_each(|(_, overlay_item)| overlay_item.update(self.counter));

        self.counter += dt;
    }

    pub fn draw(
        &mut self,
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::miniquad::Context,
        camera: &CameraComponent,
    ) -> ggez::GameResult {
        self.overlay_items
            .iter_mut()
            .enumerate()
            .filter(|(i, _)| self.active_indices[*i])
            .map(|(_, overlay_item)| overlay_item.draw(ctx, quad_ctx, camera))
            .count();

        Ok(())
    }
}

pub fn system(game_state: &mut Game, dt: f32) {
    game_state.overlay_system.update(dt);
}

pub struct OverlayItem {
    sprite: Option<SpriteComponent>,
    text: Option<String>,
    position: glam::Vec2,
    rotation: f32,
    rotation_change: f32,
    scale_fn: fn(f32) -> f32,
}

impl OverlayItem {
    pub fn new(
        sprite: Option<SpriteComponent>,
        text: Option<String>,
        position: glam::Vec2,
        rotation: f32,
        rotation_change: f32,
        scale_fn: fn(f32) -> f32,
    ) -> Self {
        Self {
            sprite,
            text,
            position,
            rotation,
            rotation_change,
            scale_fn,
        }
    }

    pub fn set_position(&mut self, position: glam::Vec2) {
        self.position = position;
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    pub fn update(&mut self, counter: f32) {
        self.rotation += self.rotation_change;

        if let Some(sprite) = &mut self.sprite {
            sprite.set_scale((self.scale_fn)(counter));
        }
    }

    pub fn draw(
        &mut self,
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::miniquad::Context,
        camera: &CameraComponent,
    ) -> ggez::GameResult {
        if let Some(sprite) = &mut self.sprite {
            return sprite_component::render_sprite(
                ctx,
                quad_ctx,
                &sprite,
                ggez::graphics::DrawParam::default()
                    .dest(camera.world_position(self.position))
                    .rotation(self.rotation),
            );
        } else if let Some(text) = &self.text {
            let txt = util::make_text(text.to_string(), 20.);
            ggez::graphics::queue_text(ctx, &txt, camera.world_position(self.position), None);
        }
        Ok(())
    }
}
