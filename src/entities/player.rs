use crate::animation_component::{AnimationComponent, AnimationState};
use crate::collision_component::AABBCollisionComponent;
use crate::move_component::MoveComponent;
use crate::particle_system::ParticleSystem;
use crate::stamina_component::StaminaComponent;
use crate::teleport_component::TeleportComponent;
use crate::transform_component::TransformComponent;
use crate::util;
use crate::Assets;
use crate::Game;
use crate::GameState;
use crate::SoundCollection;
use crate::SpriteComponent;

use crate::constants;
use crate::entities;

pub struct Player {
    pub transform: TransformComponent,
    pub animation: AnimationComponent,
    pub move_component: MoveComponent,
    pub aabb: AABBCollisionComponent,
    pub stamina: StaminaComponent,
    pub teleport: TeleportComponent,

    pub stealth_intent: bool,
    pub is_stealth: bool,
    pub was_stealth_prev: bool,
    pub footstep_timer: f32,
    pub attack_range: f32,
    pub guard_to_attack_idx: Option<usize>,

    pub move_type: MoveType,
}

#[derive(PartialEq)]
pub enum MoveType {
    Slow,
    Normal,
    Sprint,
}

impl Player {
    pub fn new(
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::miniquad::GraphicsContext,
        position: glam::Vec2,
        assets: &Assets,
        color: ggez::graphics::Color,
    ) -> Self {
        Self {
            transform: TransformComponent::new(
                position,
                constants::ENTITY_SIZE,
                util::compute_grid_index(&position),
            ),
            animation: util::build_walk_animation(
                &assets,
                util::compute_animation_duration(constants::PLAYER_SPEED),
                color,
            ),
            move_component: MoveComponent::new(constants::PLAYER_SPEED),
            aabb: AABBCollisionComponent::new(ggez::graphics::Rect::new(
                position.x,
                position.y,
                constants::ENTITY_SIZE,
                constants::ENTITY_SIZE,
            )),

            teleport: TeleportComponent::new(
                SpriteComponent::new(assets.teleport.clone(), ggez::graphics::Color::WHITE)
                    .scale(constants::SPRITE_SCALE),
            ),
            stamina: StaminaComponent::new(
                ctx,
                quad_ctx,
                100.,
                1.,
                1.,
                ggez::graphics::Rect::new(10., 10., 200., 20.),
                ggez::graphics::Color::GREEN,
            ),
            stealth_intent: false,
            is_stealth: false,
            was_stealth_prev: false,
            footstep_timer: 0.,
            attack_range: constants::PLAYER_ATTACK_RANGE,
            guard_to_attack_idx: None,

            move_type: MoveType::Normal,
        }
    }

    #[inline]
    pub fn set_stealth_intent(&mut self, stealth_intent: bool) {
        self.stealth_intent = stealth_intent;
        self.is_stealth = stealth_intent && self.stamina.stamina > 0.;
    }

    #[inline]
    pub fn set_angle(&mut self, dir: glam::Vec2) {
        if dir.length_squared() > 0. {
            self.transform.angle = util::get_vec_angle(dir);
        }
    }

    #[inline]
    pub fn set_dir(&mut self, dir: glam::Vec2) {
        self.move_component.set_x_dir(dir.x);
        self.move_component.set_y_dir(dir.y);
    }

    #[inline]
    pub fn set_x_dir(&mut self, x_dir: f32) {
        self.move_component.set_x_dir_normalized(x_dir);
    }

    #[inline]
    pub fn set_y_dir(&mut self, y_dir: f32) {
        self.move_component.set_y_dir_normalized(y_dir);
    }

    #[inline]
    pub fn set_speed(&mut self, speed: f32) {
        self.move_component.set_speed(speed);
    }

    #[inline]
    pub fn set_move_type(&mut self, move_type: MoveType) {
        self.move_type = move_type;
    }

    #[inline]
    pub fn set_colliding_vec_components(&mut self, colliding_axis: (bool, bool)) {
        self.aabb.colliding_axis = colliding_axis;
    }

    #[inline]
    pub fn is_moving(&self) -> bool {
        self.move_component.direction.length_squared() > 0.
    }

    #[inline]
    fn get_move_magnitude(&self) -> f32 {
        self.move_component.direction.length() * self.move_component.speed
    }

    fn get_sound_radius_scale(&self) -> f32 {
        (self.get_move_magnitude() * constants::SOUND_RADIUS_SCALE).powf(1.5)
    }

    pub fn get_sound_radius(&self) -> f32 {
        if self.is_stealth {
            return 0.;
        }

        self.get_sound_radius_scale() * constants::SPRITE_SIZE as f32 / 2.
    }

    pub fn teleport_action(
        &mut self,
        ctx: &mut ggez::Context,
        sound_collection: &mut SoundCollection,
        particle_system: &mut ParticleSystem,
    ) {
        if self.teleport.location.is_none()
            && self.stamina.stamina > constants::TELEPORT_COST_INTIAL
        {
            self.teleport.set_location(self.transform.clone());
            self.stamina.stamina -= constants::TELEPORT_COST_INTIAL;

            sound_collection.play(ctx, 2).unwrap_or_default();
        } else if self.stamina.stamina > constants::TELEPORT_COST {
            particle_system.emit(3, self.transform.position, 16);

            self.transform
                .set(self.teleport.location.as_ref().unwrap().position);

            self.teleport.location = None;
            self.stamina.stamina -= constants::TELEPORT_COST;

            sound_collection.play(ctx, 3).unwrap_or_default();
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.stamina
            .update(self.stealth_intent || self.move_type == MoveType::Sprint);

        if self.stamina.stamina <= 0. {
            self.is_stealth = false;

            if self.move_type == MoveType::Sprint {
                self.move_type = MoveType::Normal;
            }
        }

        if self.is_stealth {
            self.animation
                .set_color(ggez::graphics::Color::new(0., 0., 0., 0.25));

            return;
        }

        match self.move_type {
            MoveType::Slow => self.set_speed(constants::PLAYER_SPEED_SLOW),
            MoveType::Normal => self.set_speed(constants::PLAYER_SPEED),
            MoveType::Sprint => self.set_speed(constants::PLAYER_SPEED_FAST),
        }

        self.animation.set_color(ggez::graphics::Color::BLACK);

        entities::move_entity(
            &mut self.transform,
            &self.move_component,
            self.aabb.colliding_axis,
            dt,
        );
        self.set_angle(self.move_component.direction);

        self.aabb.rect.move_to(self.transform.position);

        self.animation.update(dt);

        if self.move_component.direction.length_squared() == 0. {
            self.animation.set_animation_state(AnimationState::Idle);
        } else {
            self.animation.set_animation_state(AnimationState::Active);
        }
    }
}

pub fn system(ctx: &mut ggez::Context, game_state: &mut Game, dt: f32) {
    let (player_pos, player_grid_idx, player_attack_range) = (
        game_state.player.transform.position,
        game_state.player.transform.grid_index,
        game_state.player.attack_range,
    );

    let guards = game_state.get_all_guards();

    let closest_guard = guards
        .iter()
        .enumerate()
        .filter(|(_, guard)| {
            let row = (constants::MAX_WORLD_X as usize / constants::GRID_CELL_SIZE) as isize;

            let abs_diff = (guard.transform.grid_index - player_grid_idx).abs();
            let abs_diff_bottom_row = (guard.transform.grid_index - player_grid_idx + row).abs();
            let abs_diff_top_row = (guard.transform.grid_index - player_grid_idx - row).abs();

            abs_diff <= 1 || abs_diff_bottom_row <= 1 || abs_diff_top_row <= 1
        })
        .filter(|(_, guard)| {
            !guard.is_dead()
                && (guard.transform.position - player_pos).length_squared()
                    <= player_attack_range.powi(2)
        })
        .min_by(|(_, g1), (_, g2)| {
            let dist_to_player1 = (g1.transform.position - player_pos).length_squared();
            let dist_to_player2 = (g2.transform.position - player_pos).length_squared();

            (dist_to_player1 as i32).cmp(&(dist_to_player2 as i32))
        });

    if let Some((guard_idx, guard)) = closest_guard {
        let pos = guard.transform.position;
        game_state.camera.update(pos);

        game_state.overlay_system.set_active_at(0, true);
        game_state.overlay_system.set_pos_at(0, pos);

        game_state.player.guard_to_attack_idx = Some(guard_idx);
    } else {
        game_state.overlay_system.set_active_at(0, false);

        game_state.player.guard_to_attack_idx = None;
    }

    let player = &mut game_state.player;
    player.update(dt);

    let sound_collection = &mut game_state.sound_collection;
    handle_stealth_sound(ctx, player, sound_collection);

    if player.is_stealth {
        return;
    }

    if player.is_moving() && player.footstep_timer <= 0. {
        game_state
            .particle_system
            .set_scale(1, player.get_sound_radius_scale());

        game_state
            .particle_system
            .emit(1, player.transform.position, 1);

        match player.move_type {
            MoveType::Slow => sound_collection.set_volume_to(ctx, 8, 0.1),
            MoveType::Normal => sound_collection.set_volume_to(ctx, 8, 0.5),
            MoveType::Sprint => {
                game_state
                    .particle_system
                    .emit(0, player.transform.position, 10);

                sound_collection.set_volume_to(ctx, 8, 0.9)
            }
        }
        .unwrap_or_default();

        sound_collection.play(ctx, 8).unwrap_or_default();
        player.footstep_timer = 0.33;
    }

    let target = &mut game_state.target;
    if !target.is_dead() && util::check_collision(&player.transform, &target.transform) {
        target.set_dead(true);

        sound_collection.play(ctx, 5).unwrap_or_default();

        game_state
            .particle_system
            .emit(2, target.transform.position, 50);
    }

    let exit = &mut game_state.exit;

    exit.player_exited =
        target.is_dead() && util::check_collision(&player.transform, &exit.transform);
    if exit.player_exited {
        sound_collection.play(ctx, 7).unwrap_or_default();
    }

    player.footstep_timer = (player.footstep_timer - dt).max(-1.);
}

fn handle_stealth_sound(
    ctx: &mut ggez::Context,
    player: &mut Player,
    sound_collection: &mut SoundCollection,
) {
    if player.is_stealth && !player.was_stealth_prev {
        sound_collection.play(ctx, 0).unwrap_or_default();
        player.was_stealth_prev = true;
    }
    if !player.is_stealth && player.was_stealth_prev {
        sound_collection.play(ctx, 1).unwrap_or_default();
        player.was_stealth_prev = false;
    }
}

pub fn try_attack_guard(ctx: &mut ggez::Context, game_state: &mut Game) {
    if game_state.game_state == GameState::GameOver {
        return;
    }

    if let Some(guard_to_attack_idx) = game_state.player.guard_to_attack_idx {
        let guard = &mut game_state.get_all_guards_mut()[guard_to_attack_idx];
        guard.set_dead(true);

        let pos = guard.transform.position;
        game_state.play_kill_effect(ctx, pos);
    }
}
