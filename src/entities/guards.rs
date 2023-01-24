pub mod guard_basic;
pub mod guard_heavy;
pub mod guard_scout;

use crate::animation_component::AnimationComponent;
use crate::animation_component::AnimationState;
use crate::compute_move_component::ComputeMoveComponent;
use crate::constants;
use crate::dead_component::DeadComponent;
use crate::entities;
use crate::entities::wall::Wall;
use crate::entities::AABBCollisionComponent;
use crate::look_component::LookComponent;
use crate::move_component::MoveComponent;
use crate::sprite_component::SpriteComponent;
use crate::transform_component::TransformComponent;
use crate::util;
use crate::Assets;
use crate::Game;
use crate::GameState;

use quad_rand as qrand;

pub struct Guard {
    pub guard_state: GuardState,
    pub transform: TransformComponent,
    pub animation: AnimationComponent,
    pub dead_component: DeadComponent,
    pub move_component: MoveComponent,
    pub aabb: AABBCollisionComponent,
    pub compute_move_component: ComputeMoveComponent,
    pub look_components: Vec<LookComponent>,
    pub look_idx: usize,

    pub look_color: ggez::graphics::Color,
    pub move_dir: glam::Vec2,
    pub max_move_interval: f32,
    pub move_interval: f32,
    pub wall_move_interval: f32,

    pub is_tutorial: bool,
}

#[derive(PartialEq)]
pub enum GuardState {
    HeardPlayer(glam::Vec2),
    Lookout(f32),
    Walk,
    Alert,
}

impl Guard {
    pub fn new(
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::miniquad::GraphicsContext,
        position: glam::Vec2,
        assets: &Assets,
        color: ggez::graphics::Color,
        is_tutorial: bool,
    ) -> Self {
        Self {
            guard_state: GuardState::Walk,
            transform: TransformComponent::new(
                position,
                constants::ENTITY_SIZE,
                util::compute_grid_index(&position),
            ),
            animation: util::build_walk_animation(
                &assets,
                util::compute_animation_duration(constants::GUARD_SPEED),
                color,
            ),
            dead_component: DeadComponent::new(
                SpriteComponent::new(
                    assets.dead.clone(),
                    ggez::graphics::Color::new(0.5, 0.5, 0.5, 0.75),
                )
                .scale(constants::SPRITE_SCALE),
                false,
            ),
            move_component: MoveComponent::new(constants::GUARD_SPEED),
            aabb: AABBCollisionComponent::new(ggez::graphics::Rect::new(
                position.x,
                position.y,
                constants::ENTITY_SIZE,
                constants::ENTITY_SIZE,
            )),
            compute_move_component: ComputeMoveComponent::new(
                ctx,
                quad_ctx,
                8,
                constants::GUARD_VIEW_DISTANCE,
            ),
            look_components: vec![LookComponent::new_with_mesh(
                ctx,
                quad_ctx,
                glam::vec2(0., 1.),
                constants::GUARD_FOV,
                constants::GUARD_VIEW_DISTANCE,
                constants::N_FOV_RAYS,
            )],
            look_idx: 0,
            look_color: ggez::graphics::Color::WHITE,
            move_dir: glam::Vec2::ZERO,
            max_move_interval: 0.,
            move_interval: 0.,
            wall_move_interval: 0.,

            is_tutorial,
        }
    }

    #[inline]
    fn set_angle(&mut self, dir: glam::Vec2) {
        if dir.length_squared() > 0. {
            self.transform.angle = util::get_vec_angle(dir);
        }

        self.look_components[self.look_idx].look_at = dir;
    }

    #[inline]
    fn add_look_component(&mut self, look_component: LookComponent) {
        self.look_components.push(look_component);
    }

    #[inline]
    fn next_look_component(&mut self) {
        self.look_idx = (self.look_idx + 1) % self.look_components.len();
    }

    #[inline]
    fn set_speed(&mut self, speed: f32) {
        self.move_component.speed = speed;
    }

    #[inline]
    fn set_look_color(&mut self, look_color: ggez::graphics::Color) {
        self.look_color = look_color;
    }

    #[inline]
    pub fn set_colliding_vec_components(&mut self, colliding_axis: (bool, bool)) {
        self.aabb.colliding_axis = colliding_axis;
    }

    #[inline]
    pub fn is_dead(&self) -> bool {
        self.dead_component.is_dead
    }

    #[inline]
    pub fn set_dead(&mut self, is_dead: bool) {
        self.dead_component.is_dead = is_dead;
    }

    pub fn get_curr_animation_frame(&self) -> &SpriteComponent {
        if self.dead_component.is_dead {
            return &self.dead_component.sprite;
        }

        self.animation.get_curr_frame()
    }

    fn set_lookout(
        &mut self,
        speed_low: f32,
        speed_high: f32,
        duration_low: f32,
        duration_high: f32,
    ) {
        let lookout_speed = qrand::gen_range(speed_low, speed_high)
            * (qrand::gen_range::<f32>(0., 1.) >= 0.5)
                .then(|| 1.)
                .unwrap_or(-1.);

        self.guard_state = GuardState::Lookout(lookout_speed);

        self.max_move_interval = qrand::gen_range(duration_low, duration_high);
        self.move_interval = self.max_move_interval;
    }

    fn do_move(
        &mut self,
        rect_objects: &Vec<(&ggez::graphics::Rect, isize)>,
        move_interval_low: f32,
        move_interval_high: f32,
    ) {
        let min_ray_scale = self.look_components[self.look_idx]
            .ray_scales
            .iter()
            .min_by(|&&a, &&b| ((a * 100.) as i32).cmp(&((b * 100.) as i32)))
            .unwrap_or(&1.);

        let is_close_to_wall = *min_ray_scale < 0.8;

        if self.move_interval <= 0. || is_close_to_wall && self.wall_move_interval <= 0. {
            let new_dir = self
                .compute_move_component
                .get_move_direction(&self.transform, rect_objects)
                .normalize_or_zero();

            self.move_dir = new_dir;

            self.max_move_interval = qrand::gen_range(move_interval_low, move_interval_high);
            self.move_interval = self.max_move_interval;

            if is_close_to_wall {
                self.wall_move_interval = if self.guard_state == GuardState::Walk {
                    0.5
                } else {
                    0.25
                };
            }
        } else {
            let lerped_dir = if self.max_move_interval != 0. {
                util::vec_lerp(
                    self.move_component.direction,
                    self.move_dir,
                    1. - self.move_interval / self.max_move_interval,
                )
            } else {
                self.move_component.direction
            };

            self.move_component.set_direction_normalized(lerped_dir);
            self.set_angle(self.move_component.direction);
        }
    }

    fn do_heard_player(&mut self, dir_to_player: glam::Vec2) {
        self.set_speed(0.);

        let lerp_to_player = util::vec_lerp(
            self.move_dir,
            dir_to_player,
            1. - self.move_interval / self.max_move_interval,
        );

        self.move_dir = lerp_to_player;
        self.move_component.set_direction(lerp_to_player);

        self.set_angle(self.move_component.direction);

        if self.move_interval <= 0. {
            self.set_lookout(0.01, 0.7, 3., 4.);
        }
    }

    fn do_lookout(&mut self, lookout_dir: glam::Vec2) {
        self.set_speed(0.);
        self.move_dir = lookout_dir;

        self.move_component.set_direction(lookout_dir);
        self.set_angle(self.move_component.direction);
    }

    fn update(&mut self, dt: f32, player_sound: &TransformComponent) {
        if self.dead_component.is_dead {
            return;
        }

        entities::move_entity(
            &mut self.transform,
            &self.move_component,
            self.aabb.colliding_axis,
            dt,
        );

        self.aabb.rect.move_to(self.transform.position);

        self.animation.update(dt);

        if self.move_component.speed == 0. {
            self.animation.set_animation_state(AnimationState::Idle);
        } else {
            self.animation.set_animation_state(AnimationState::Active);
        }

        if !matches!(self.guard_state, GuardState::HeardPlayer(_))
            && util::check_collision(&self.transform, player_sound)
        {
            let dir_to_player =
                (player_sound.position - self.transform.position).normalize_or_zero();
            self.guard_state = GuardState::HeardPlayer(dir_to_player);

            self.max_move_interval = qrand::gen_range(0.3, 0.4);
            self.move_interval = self.max_move_interval;
        }

        self.move_interval = (self.move_interval - dt).max(-1.);
        self.wall_move_interval = (self.wall_move_interval - dt).max(-1.);
    }
}

pub fn alert_all(ctx: &mut ggez::Context, game_state: &mut Game) {
    game_state
        .get_all_guards_mut()
        .iter_mut()
        .for_each(|guard| {
            guard.guard_state = GuardState::Alert;
            guard.set_look_color(ggez::graphics::Color::RED);
        });

    game_state.are_guards_alerted = true;
    game_state.sound_collection.play(ctx, 6).unwrap_or_default();
}

pub fn system(ctx: &mut ggez::Context, game_state: &mut Game, dt: f32) {
    if is_transform_detected(
        &game_state.walls,
        &game_state.get_all_guards(),
        &game_state.player.transform,
        game_state.player.is_stealth,
    ) {
        game_state.game_state = GameState::GameOver;
        game_state.sound_collection.play(ctx, 4).unwrap_or_default();
    }

    if !game_state.are_guards_alerted
        && game_state.target.is_dead()
        && is_transform_detected(
            &game_state.walls,
            &game_state.get_all_guards(),
            &game_state.target.transform,
            false,
        )
    {
        alert_all(ctx, game_state);
    }

    let guards = game_state.get_all_guards();
    let alive_guards = game_state
        .get_all_guards()
        .iter()
        .filter(|guard| !guard.is_dead())
        .map(|guard| *guard)
        .collect::<Vec<&Guard>>();
    let dead_guards = guards.iter().filter(|guard| guard.is_dead());

    let dead_guard_detected = dead_guards
        .map(|dead_guard| {
            is_transform_detected(
                &game_state.walls,
                &alive_guards,
                &dead_guard.transform,
                false,
            )
        })
        .any(|is_detected| is_detected);

    if !game_state.are_guards_alerted && dead_guard_detected {
        alert_all(ctx, game_state);
    }

    let aabb_objects = game_state
        .walls
        .iter()
        .map(|wall| (&wall.aabb.rect, wall.transform.grid_index))
        .collect::<Vec<(&ggez::graphics::Rect, isize)>>();

    let (p_position, p_sound_radius) = (
        game_state.player.transform.position,
        game_state.player.get_sound_radius(),
    );
    let player_sound = TransformComponent::new(p_position, p_sound_radius, -1);

    game_state
        .guards_basic
        .iter_mut()
        .for_each(|guard| guard.update(dt, &aabb_objects, &player_sound));

    game_state
        .guards_scout
        .iter_mut()
        .for_each(|guard| guard.update(dt, &aabb_objects, &player_sound));

    game_state
        .guards_heavy
        .iter_mut()
        .for_each(|guard| guard.update(dt, &aabb_objects, &player_sound));
}

pub fn is_transform_detected(
    walls: &Vec<Wall>,
    guards: &Vec<&Guard>,
    transform: &TransformComponent,
    is_stealth: bool,
) -> bool {
    // If transform is stealth, it cannot be detected
    if is_stealth {
        return false;
    }

    let aabb_objects = walls
        .iter()
        .map(|wall| &wall.aabb.rect)
        .collect::<Vec<&ggez::graphics::Rect>>();

    guards.iter().any(|guard| {
        !guard.is_dead()
            && util::check_spotted(
                &guard.look_components[guard.look_idx],
                &guard.transform,
                transform,
                &aabb_objects,
            )
    })
}
