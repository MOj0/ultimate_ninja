mod animation_component;
mod assets;
mod camera_component;
mod collision_component;
mod compute_move_component;
mod constants;
mod dead_component;
mod entities;
mod level;
mod look_component;
mod mouse_input_handler;
mod move_component;
mod network_system;
mod overlay_system;
mod particle_system;
mod sound_collection;
mod sprite_component;
mod stamina_component;
mod teleport_component;
mod tile_component;
mod transform_component;
mod util;

use crate::assets::Assets;
use crate::mouse_input_handler::MouseInputHandler;
use crate::network_system::NetworkSystem;
use crate::sound_collection::SoundCollection;
use crate::sprite_component::SpriteComponent;
use crate::transform_component::TransformComponent;

extern crate good_web_game as ggez;

use ggez::event::{KeyCode, KeyMods};
use ggez::input::MouseButton;
use ggez::miniquad;
use ggez::{audio, graphics, Context, GameResult};

// TODO: Improve guard movement
// TODO: Scaling for different resolutions (handle camera - ortographic...)

#[derive(PartialEq)]
pub enum GameState {
    Menu,
    Info,
    Leaderboard,
    SubmitTime,
    Game,
    Pause,
    LevelAnimation,
    GameOver,
    EndScreen,
}

pub struct Game {
    game_state: GameState,
    assets: assets::Assets,
    camera: camera_component::CameraComponent,
    player: entities::player::Player,
    target: entities::target::Target,
    guards_basic: Vec<entities::guards::guard_basic::GuardBasic>,
    guards_scout: Vec<entities::guards::guard_scout::GuardScout>,
    guards_heavy: Vec<entities::guards::guard_heavy::GuardHeavy>,
    walls: Vec<entities::wall::Wall>,
    floor_tiles: Vec<tile_component::TileComponent>,
    exit: entities::exit::Exit,
    double_press_timer: Option<f32>,
    sound_collection: SoundCollection,
    mouse_input_handler: MouseInputHandler,
    particle_system: particle_system::ParticleSystem,
    overlay_system: overlay_system::OverlaySystem,
    level_idx: usize,
    are_guards_alerted: bool,
    debug_draw: bool,
    is_touch_joystick_activated: bool,
    menu_rectangle: graphics::Mesh,
    menu_square: graphics::Mesh,
    grid_line: graphics::Mesh,
    n_objects: usize,
    curr_level_time: f32,
    level_times: [f32; level::LEVEL_COUNT],
    leaderboard_str: Option<String>,
    network_system: NetworkSystem,
    tokio_runtime: tokio::runtime::Runtime,
    player_name: String,
    is_skip_tutorial: bool,
}

impl Game {
    pub fn new(ctx: &mut Context, quad_ctx: &mut miniquad::GraphicsContext) -> Self {
        let (is_muted, are_particles_activated, is_skip_tutorial) =
            util::read_config(&util::config_filename());

        let tokio_runtime = tokio::runtime::Runtime::new().unwrap();

        let game_state = GameState::Menu;

        let assets = Assets::load(ctx, quad_ctx);

        let camera = camera_component::CameraComponent::new(
            glam::Vec2::ZERO,
            glam::vec2(
                (constants::WIDTH / 2) as f32,
                (constants::HEIGHT / 2) as f32,
            ),
            constants::CAMERA_DEFAULT_LERP_DELTA,
        );

        let player = entities::player::Player::new(
            ctx,
            quad_ctx,
            glam::Vec2::ZERO,
            &assets,
            ggez::graphics::Color::BLACK,
        );

        let target =
            entities::target::Target::new(glam::Vec2::ZERO, &assets, ggez::graphics::Color::GREEN);

        let exit = entities::exit::Exit::new(
            glam::Vec2::ZERO,
            SpriteComponent::new(assets.exit.clone(), ggez::graphics::Color::WHITE),
        );

        let mut sounds = [
            audio::Source::new(ctx, "sounds/stealth.ogg").unwrap(),
            audio::Source::new(ctx, "sounds/unstealth.ogg").unwrap(),
            audio::Source::new(ctx, "sounds/teleport_initial.ogg").unwrap(),
            audio::Source::new(ctx, "sounds/teleport.ogg").unwrap(),
            audio::Source::new(ctx, "sounds/player_dead.ogg").unwrap(),
            audio::Source::new(ctx, "sounds/target_killed.ogg").unwrap(),
            audio::Source::new(ctx, "sounds/dead_target_detected.ogg").unwrap(),
            audio::Source::new(ctx, "sounds/level_exit.ogg").unwrap(),
            audio::Source::new(ctx, "sounds/footstep.ogg").unwrap(),
        ];
        sounds
            .iter_mut()
            .for_each(|sound| sound.set_volume(ctx, 0.5).unwrap());

        let sound_collection = SoundCollection {
            sounds,
            is_on: !is_muted,
        };

        let mouse_input_handler = MouseInputHandler::new(
            ctx,
            quad_ctx,
            glam::vec2(
                (constants::WIDTH - 125) as f32,
                (constants::HEIGHT - 125) as f32,
            ),
            80.,
        );

        let mut particle_system = particle_system::ParticleSystem::new(are_particles_activated);
        let particle_image = util::make_particle_image(ctx, quad_ctx);

        let player_move_particle_emitter = particle_system::ParticleEmitter::new(
            glam::Vec2::ZERO,
            0.05,
            0.5,
            0.,
            ggez::graphics::Color::WHITE,
            2.,
            100,
            particle_image.clone(),
        );

        let player_footstep_particle_emitter = particle_system::ParticleEmitter::new(
            glam::Vec2::ZERO,
            0.1,
            0.12,
            0.01,
            ggez::graphics::Color::new(1., 1., 1., 0.5),
            0.75,
            2,
            assets.footstep.clone(),
        );

        let target_killed_particle_emitter = particle_system::ParticleEmitter::new(
            glam::Vec2::ZERO,
            0.5,
            1.,
            0.,
            ggez::graphics::Color::RED,
            5.,
            100,
            particle_image.clone(),
        );

        let teleport_particle_emitter = particle_system::ParticleEmitter::new(
            glam::Vec2::ZERO,
            0.1,
            0.5,
            0.,
            ggez::graphics::Color::WHITE,
            5.,
            100,
            particle_image.clone(),
        );

        particle_system.add_emitter(player_move_particle_emitter);
        particle_system.add_emitter(player_footstep_particle_emitter);
        particle_system.add_emitter(target_killed_particle_emitter);
        particle_system.add_emitter(teleport_particle_emitter);

        let mut overlay_system = overlay_system::OverlaySystem::new();

        let marker =
            sprite_component::SpriteComponent::new(assets.marker.clone(), graphics::Color::WHITE)
                .scale(glam::Vec2::splat(0.4));
        let overlay_marker = overlay_system::OverlayItem::new(
            Some(marker),
            None,
            glam::vec2(0., 0.),
            0.,
            0.03,
            |_: f32| 0.4,
        );

        let arrow =
            sprite_component::SpriteComponent::new(assets.arrow.clone(), graphics::Color::WHITE);
        let overlay_arrow = overlay_system::OverlayItem::new(
            Some(arrow),
            None,
            glam::vec2(0., 0.),
            0.,
            0.,
            |x: f32| x.sin().abs() * 0.3 + 0.7,
        );

        let overlay_text = overlay_system::OverlayItem::new(
            None,
            Some("".to_owned()),
            glam::vec2(0., 0.),
            0.,
            0.,
            |_: f32| 1.,
        );

        overlay_system.add_item(overlay_marker);
        overlay_system.add_item(overlay_arrow);
        overlay_system.add_item(overlay_text);

        let level_idx = 0;

        let menu_rectangle = graphics::Mesh::new_rounded_rectangle(
            ctx,
            quad_ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0., 0., constants::BTN_DIM_RECT.x, constants::BTN_DIM_RECT.y),
            10.,
            graphics::Color::new(0.25, 0.25, 0.25, 0.6),
        )
        .unwrap();

        let menu_square = graphics::Mesh::new_rounded_rectangle(
            ctx,
            quad_ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                0.,
                0.,
                constants::BTN_DIM_SQUARE.x,
                constants::BTN_DIM_SQUARE.y,
            ),
            10.,
            graphics::Color::new(0.4, 0.4, 0.4, 0.8),
        )
        .unwrap();

        let grid_line = graphics::Mesh::new_line(
            ctx,
            quad_ctx,
            &[
                glam::vec2(0., 0.),
                glam::vec2(
                    constants::MAX_WORLD_X.max(constants::MAX_WORLD_Y) as f32,
                    0.,
                ),
            ],
            2.,
            graphics::Color::new(0.85, 0.85, 0.85, 0.75),
        )
        .unwrap();

        let network_system = NetworkSystem::new();

        Game {
            game_state,
            assets,
            camera,
            player,
            target,
            guards_basic: vec![],
            guards_scout: vec![],
            guards_heavy: vec![],
            walls: vec![],
            floor_tiles: vec![],
            exit,
            double_press_timer: None,
            sound_collection,
            mouse_input_handler,
            particle_system,
            overlay_system,
            level_idx,
            are_guards_alerted: false,
            debug_draw: false,
            is_touch_joystick_activated: false,
            menu_rectangle,
            menu_square,
            grid_line,
            n_objects: 0,
            curr_level_time: 0.,
            level_times: [0.; level::LEVEL_COUNT],
            leaderboard_str: None,
            network_system,
            tokio_runtime,
            player_name: String::new(),
            is_skip_tutorial,
        }
    }

    pub fn reset_state(&mut self, is_proceed: bool) {
        if self.game_state == GameState::GameOver {
            self.game_state = GameState::Game;
        }

        self.player.is_stealth = false;
        self.player.stealth_intent = false;

        self.target.set_dead(false);
        self.are_guards_alerted = false;

        self.exit.player_exited = false;

        self.guards_basic.clear();
        self.guards_scout.clear();
        self.guards_heavy.clear();

        self.walls.clear();

        self.floor_tiles.clear();

        self.particle_system.reset();

        if is_proceed {
            self.curr_level_time = 0.;
        }
    }

    // Collect all common guard objects into one vector
    pub fn get_all_guards(&self) -> Vec<&entities::guards::Guard> {
        let guards_b = self
            .guards_basic
            .iter()
            .map(|guard_basic| &guard_basic.guard);

        let guards_s = self
            .guards_scout
            .iter()
            .map(|guard_sniper| &guard_sniper.guard);

        let guards_h = self
            .guards_heavy
            .iter()
            .map(|guard_heavy| &guard_heavy.guard);

        guards_b
            .chain(guards_s)
            .chain(guards_h)
            .collect::<Vec<&entities::guards::Guard>>()
    }

    pub fn get_all_guards_mut(&mut self) -> Vec<&mut entities::guards::Guard> {
        let guards_b = self
            .guards_basic
            .iter_mut()
            .map(|guard_basic| &mut guard_basic.guard);

        let guards_s = self
            .guards_scout
            .iter_mut()
            .map(|guard_sniper| &mut guard_sniper.guard);

        let guards_h = self
            .guards_heavy
            .iter_mut()
            .map(|guard_heavy| &mut guard_heavy.guard);

        guards_b
            .chain(guards_s)
            .chain(guards_h)
            .collect::<Vec<&mut entities::guards::Guard>>()
    }

    pub fn play_kill_effect(&mut self, ctx: &mut Context, pos: glam::Vec2) {
        self.sound_collection.play(ctx, 5).unwrap_or_default();
        self.particle_system.emit(2, pos, 50);
    }

    fn next_level(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::Context,
        is_proceeed: bool,
    ) {
        self.level_times[self.level_idx] = self.curr_level_time;

        if self.level_idx == level::LEVEL_COUNT - 1 {
            self.reset_state(is_proceeed);
            self.level_idx = 0;
            self.game_state = GameState::EndScreen;
        } else {
            self.level_idx += 1;
            level::load_level(ctx, quad_ctx, self, self.level_idx, is_proceeed);

            self.n_objects =
                1 + self.guards_basic.len() + self.guards_scout.len() + self.walls.len();
            self.game_state = GameState::LevelAnimation;
            self.curr_level_time = constants::LEVEL_ANIMATION_TIME;
        }
    }

    fn do_level_animation(&mut self, dt: f32) {
        if self.curr_level_time <= 0.
            && (self.player.is_moving() || self.level_idx < level::TUTORIAL_COUNT)
        {
            self.camera
                .set_lerp_delta(constants::CAMERA_DEFAULT_LERP_DELTA);

            self.curr_level_time = 0.;
            self.game_state = GameState::Game;

            return;
        }

        self.camera.set_lerp_delta(0.05);
        if self.curr_level_time >= constants::LEVEL_ANIMATION_TIME / 3. {
            self.camera.update(self.target.transform.position);
        } else {
            self.camera.update(self.player.transform.position);
        }

        self.curr_level_time = (self.curr_level_time - dt).max(0.);
    }

    fn draw_menu(
        &self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::Context,
    ) -> Result<(), ggez::GameError> {
        graphics::draw(
            ctx,
            quad_ctx,
            &util::make_text("Ultimate Ninja".into(), 42.),
            graphics::DrawParam::default().dest(glam::vec2(
                constants::WIDTH as f32 * 0.3525,
                constants::HEIGHT as f32 * 0.0833,
            )),
        )?;

        graphics::draw(
            ctx,
            quad_ctx,
            &self.menu_rectangle,
            graphics::DrawParam::default().dest(constants::BTN_PLAY_POS),
        )?;
        graphics::draw(
            ctx,
            quad_ctx,
            &util::make_text("Play".into(), 36.),
            graphics::DrawParam::default().dest(
                constants::BTN_PLAY_POS
                    + glam::vec2(
                        constants::WIDTH as f32 * 0.115,
                        constants::HEIGHT as f32 * 0.033,
                    ),
            ),
        )?;

        graphics::draw(
            ctx,
            quad_ctx,
            &self.menu_rectangle,
            graphics::DrawParam::default().dest(constants::BTN_INFO_POS),
        )?;
        graphics::draw(
            ctx,
            quad_ctx,
            &util::make_text("Info".into(), 36.),
            graphics::DrawParam::default().dest(
                constants::BTN_INFO_POS
                    + glam::vec2(
                        constants::WIDTH as f32 * 0.115,
                        constants::HEIGHT as f32 * 0.033,
                    ),
            ),
        )?;

        graphics::draw(
            ctx,
            quad_ctx,
            &self.menu_rectangle,
            graphics::DrawParam::default().dest(constants::BTN_BOTTOM_RIGHT_POS),
        )?;

        graphics::draw(
            ctx,
            quad_ctx,
            &util::make_text("Leaderboard".into(), 36.),
            graphics::DrawParam::default().dest(
                constants::BTN_BOTTOM_RIGHT_POS
                    + glam::vec2(
                        constants::WIDTH as f32 * 0.03425,
                        constants::HEIGHT as f32 * 0.033,
                    ),
            ),
        )?;

        graphics::draw(
            ctx,
            quad_ctx,
            &self.assets.ultimate_ninja,
            graphics::DrawParam::default().dest(glam::vec2(
                constants::WIDTH as f32 * 0.1875,
                constants::HEIGHT as f32 * 0.33,
            )),
        )?;

        graphics::draw(
            ctx,
            quad_ctx,
            &self.menu_square,
            graphics::DrawParam::default().dest(constants::BTN_BOTTOM_LEFT_POS1),
        )?;
        if !self.sound_collection.is_on {
            graphics::draw(
                ctx,
                quad_ctx,
                &self.assets.checkmark,
                graphics::DrawParam::default()
                    .dest(constants::BTN_BOTTOM_LEFT_POS1 + glam::vec2(5., 5.))
                    .scale(glam::Vec2::splat(constants::WIDTH as f32 / 800.)),
            )?;
        }
        graphics::draw(
            ctx,
            quad_ctx,
            &util::make_text("Mute".into(), 36.),
            graphics::DrawParam::default().dest(
                constants::BTN_BOTTOM_LEFT_POS1
                    + glam::vec2(
                        constants::WIDTH as f32 * 0.075,
                        constants::HEIGHT as f32 * 0.0166,
                    ),
            ),
        )?;

        graphics::draw(
            ctx,
            quad_ctx,
            &self.menu_square,
            graphics::DrawParam::default().dest(constants::BTN_BOTTOM_LEFT_POS2),
        )?;
        if self.particle_system.is_activated {
            graphics::draw(
                ctx,
                quad_ctx,
                &self.assets.checkmark,
                graphics::DrawParam::default()
                    .dest(constants::BTN_BOTTOM_LEFT_POS2 + glam::vec2(5., 5.))
                    .scale(glam::Vec2::splat(constants::WIDTH as f32 / 800.)),
            )?;
        }
        graphics::draw(
            ctx,
            quad_ctx,
            &util::make_text("Particles".into(), 36.),
            graphics::DrawParam::default().dest(
                constants::BTN_BOTTOM_LEFT_POS2
                    + glam::vec2(
                        constants::WIDTH as f32 * 0.075,
                        constants::HEIGHT as f32 * 0.0166,
                    ),
            ),
        )?;

        graphics::draw(
            ctx,
            quad_ctx,
            &self.menu_square,
            graphics::DrawParam::default().dest(constants::BTN_BOTTOM_LEFT_POS3),
        )?;
        if self.is_skip_tutorial {
            graphics::draw(
                ctx,
                quad_ctx,
                &self.assets.checkmark,
                graphics::DrawParam::default()
                    .dest(constants::BTN_BOTTOM_LEFT_POS3 + glam::vec2(5., 5.))
                    .scale(glam::Vec2::splat(constants::WIDTH as f32 / 800.)),
            )?;
        }
        graphics::draw(
            ctx,
            quad_ctx,
            &util::make_text("Skip tutorial".into(), 36.),
            graphics::DrawParam::default().dest(
                constants::BTN_BOTTOM_LEFT_POS3
                    + glam::vec2(
                        constants::WIDTH as f32 * 0.075,
                        constants::HEIGHT as f32 * 0.0166,
                    ),
            ),
        )?;

        Ok(())
    }

    fn draw_info(
        &self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::Context,
    ) -> Result<(), ggez::GameError> {
        graphics::draw(
            ctx,
            quad_ctx,
            &util::make_text("Ultimate Ninja".into(), 42.),
            graphics::DrawParam::default().dest(glam::vec2(
                constants::WIDTH as f32 * 0.3525,
                constants::HEIGHT as f32 * 0.0833,
            )),
        )?;

        graphics::draw(
            ctx,
            quad_ctx,
            &self.menu_rectangle,
            graphics::DrawParam::default().dest(constants::BTN_BACK_POS),
        )?;
        graphics::draw(
            ctx,
            quad_ctx,
            &util::make_text("Back".into(), 36.),
            graphics::DrawParam::default().dest(
                constants::BTN_BACK_POS
                    + glam::vec2(
                        constants::WIDTH as f32 * 0.1,
                        constants::HEIGHT as f32 * 0.033,
                    ),
            ),
        )?;

        graphics::draw(
            ctx,
            quad_ctx,
            &self.menu_rectangle,
            graphics::DrawParam::default()
                .dest(glam::vec2(
                    constants::WIDTH as f32 * 0.035,
                    constants::HEIGHT as f32 * 0.4,
                ))
                .scale(glam::vec2(3.2, 4.35)),
        )?;
        graphics::draw(
            ctx,
            quad_ctx,
            &util::make_text(
                "Eliminate the green target\n
Do not get spotted by guards\n
Move with WASD or arrow keys\n
Sprint with Shift, sneak with Ctrl\n
Hold F key for stealth\n
Double-press F to place a teleport marker at your position\n
Double-click F again to teleport there\n
Be wary of your stamina\n
When you complete your mission, a pathway to the next level will appear"
                    .into(),
                18.,
            ),
            graphics::DrawParam::default().dest(glam::vec2(
                constants::WIDTH as f32 * 0.08,
                constants::HEIGHT as f32 * 0.42,
            )),
        )?;

        Ok(())
    }

    fn draw_leaderboard(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::Context,
    ) -> Result<(), ggez::GameError> {
        graphics::draw(
            ctx,
            quad_ctx,
            &util::make_text("Leaderboard".into(), 42.),
            graphics::DrawParam::default().dest(glam::vec2(
                constants::WIDTH as f32 * 0.35,
                constants::HEIGHT as f32 * 0.0833,
            )),
        )?;

        graphics::draw(
            ctx,
            quad_ctx,
            &self.menu_rectangle,
            graphics::DrawParam::default().dest(constants::BTN_BACK_POS),
        )?;
        graphics::draw(
            ctx,
            quad_ctx,
            &util::make_text("Back".into(), 36.),
            graphics::DrawParam::default().dest(
                constants::BTN_BACK_POS
                    + glam::vec2(
                        constants::WIDTH as f32 * 0.1,
                        constants::HEIGHT as f32 * 0.033,
                    ),
            ),
        )?;

        graphics::draw(
            ctx,
            quad_ctx,
            &self.menu_rectangle,
            graphics::DrawParam::default()
                .dest(glam::vec2(
                    constants::WIDTH as f32 * 0.0375,
                    constants::HEIGHT as f32 * 0.4166,
                ))
                .scale(glam::vec2(3., 4.55)),
        )?;

        if self.leaderboard_str.is_none()
            && self.network_system.submit_finished()
            && self.network_system.leaderboard_ready()
        {
            let mut network = NetworkSystem::new();
            std::mem::swap(&mut network, &mut self.network_system);

            self.leaderboard_str = Some(network.get_response());
        }

        let leaderboard_str = self
            .leaderboard_str
            .clone()
            .unwrap_or("Requesting leaderboard...".to_owned());

        graphics::draw(
            ctx,
            quad_ctx,
            &util::make_text(leaderboard_str, 18.),
            graphics::DrawParam::default().dest(glam::vec2(
                constants::WIDTH as f32 * 0.1,
                constants::HEIGHT as f32 * 0.45,
            )),
        )?;

        self.curr_level_time += ggez::timer::delta(ctx).as_secs_f32();

        Ok(())
    }

    fn draw_end_screen(
        &self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::Context,
    ) -> Result<(), ggez::GameError> {
        graphics::draw(
            ctx,
            quad_ctx,
            &self.menu_rectangle,
            graphics::DrawParam::default().dest(constants::BTN_BOTTOM_LEFT_POS),
        )?;
        graphics::draw(
            ctx,
            quad_ctx,
            &util::make_text("Menu".into(), 36.),
            graphics::DrawParam::default()
                .dest(constants::BTN_BOTTOM_LEFT_POS + glam::vec2(80., 20.)),
        )?;

        graphics::draw(
            ctx,
            quad_ctx,
            &self.menu_rectangle,
            graphics::DrawParam::default().dest(constants::BTN_BOTTOM_RIGHT_POS),
        )?;
        graphics::draw(
            ctx,
            quad_ctx,
            &util::make_text("Submit".into(), 36.),
            graphics::DrawParam::default()
                .dest(constants::BTN_BOTTOM_RIGHT_POS + glam::vec2(65., 20.)),
        )?;

        let level_times_str = (level::TUTORIAL_COUNT..level::LEVEL_COUNT)
            .map(|lvl_idx| {
                format!(
                    "Level {}: {}\n",
                    lvl_idx + 1 - level::TUTORIAL_COUNT,
                    self.level_times[lvl_idx]
                )
            })
            .reduce(|acc, itm| acc + &itm)
            .unwrap();

        graphics::draw(
            ctx,
            quad_ctx,
            &util::make_text(level_times_str, 24.),
            graphics::DrawParam::default().dest(glam::vec2(250., 20.)),
        )?;

        let total_time = self
            .level_times
            .iter()
            .skip(level::TUTORIAL_COUNT)
            .sum::<f32>();
        graphics::draw(
            ctx,
            quad_ctx,
            &util::make_text(format!("Total time: {:.3}", total_time), 32.),
            graphics::DrawParam::default().dest(glam::vec2(225., 325.)),
        )?;

        graphics::draw(
            ctx,
            quad_ctx,
            &util::make_text("Enter username:".into(), 32.),
            graphics::DrawParam::default().dest(glam::vec2(200., 375.)),
        )?;

        graphics::draw(
            ctx,
            quad_ctx,
            &self.menu_rectangle,
            graphics::DrawParam::default()
                .dest(glam::vec2(180., 410.))
                .scale(glam::vec2(2., 1.)),
        )?;
        graphics::draw(
            ctx,
            quad_ctx,
            &util::make_text(format!("{}", self.player_name), 32.),
            graphics::DrawParam::default().dest(glam::vec2(200., 430.)),
        )?;
        Ok(())
    }

    fn draw_game(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::Context,
    ) -> Result<(), ggez::GameError> {
        use graphics::DrawParam;
        let mut n_objects_drawn = 0;

        self.floor_tiles
            .iter()
            .map(|floor_tile| {
                sprite_component::render(
                    ctx,
                    quad_ctx,
                    &floor_tile.sprite,
                    DrawParam::default()
                        .dest(self.camera.world_position(floor_tile.transform.position))
                        .scale(floor_tile.sprite.scale)
                        .color(graphics::Color::new(
                            floor_tile.brightness,
                            floor_tile.brightness
                                * self.are_guards_alerted.then(|| 0.5).unwrap_or(1.),
                            floor_tile.brightness
                                * self.are_guards_alerted.then(|| 0.5).unwrap_or(1.),
                            1.,
                        )),
                )
            })
            .count();

        if self.player.teleport.location.is_some() {
            sprite_component::render_sprite(
                ctx,
                quad_ctx,
                &self.player.teleport.sprite,
                DrawParam::default().dest(
                    self.camera
                        .world_position(self.player.teleport.location.as_ref().unwrap().position),
                ),
            )?;
        }

        sprite_component::render_sprite(
            ctx,
            quad_ctx,
            &self.player.animation.get_curr_frame(),
            DrawParam::default()
                .dest(self.camera.world_position(self.player.transform.position))
                .rotation(-self.player.transform.angle),
        )?;

        if self.camera.contains(&self.target.aabb) {
            sprite_component::render_sprite(
                ctx,
                quad_ctx,
                &self.target.get_curr_animation_frame(),
                DrawParam::default()
                    .dest(self.camera.world_position(self.target.transform.position))
                    .rotation(-self.target.transform.angle),
            )?;
            n_objects_drawn += 1;
        }

        let all_guards = self.get_all_guards();

        // Draw guards
        n_objects_drawn += all_guards
            .iter()
            .filter(|guard| self.camera.contains(&guard.aabb))
            .map(|guard| {
                sprite_component::render_sprite(
                    ctx,
                    quad_ctx,
                    &guard.get_curr_animation_frame(),
                    DrawParam::default()
                        .dest(self.camera.world_position(guard.transform.position))
                        .rotation(-guard.transform.angle),
                )
            })
            .count();

        // Draw look mesh compositions
        if self.game_state != GameState::LevelAnimation {
            all_guards
                .iter()
                .filter(|guard| !guard.is_dead())
                .map(|guard| {
                    guard.look_components[guard.look_idx]
                        .fov_mesh_composition
                        .iter()
                        .zip(&guard.look_components[guard.look_idx].ray_scales)
                        .flat_map(|(fov_section, scale)| {
                            sprite_component::render_mesh(
                                ctx,
                                quad_ctx,
                                fov_section,
                                DrawParam::default()
                                    .dest(self.camera.world_position(guard.transform.position))
                                    .rotation(
                                        -constants::PI / 2.
                                            - util::get_vec_angle(
                                                guard.look_components[guard.look_idx].look_at,
                                            ),
                                    )
                                    .scale(glam::Vec2::splat(*scale))
                                    .color(guard.look_color),
                            )
                        })
                        .count()
                })
                .count();
        }

        if self.debug_draw {
            // Draw look rays
            all_guards
                .iter()
                .map(|guard| {
                    guard.look_components[guard.look_idx]
                        .ray_lines
                        .iter()
                        .enumerate()
                        .map(|(ray_idx, ray_line)| {
                            sprite_component::render_mesh(
                                ctx,
                                quad_ctx,
                                ray_line,
                                DrawParam::default()
                                    .dest(self.camera.world_position(guard.transform.position))
                                    .rotation(
                                        -constants::PI / 2.
                                            - util::get_vec_angle(
                                                guard.look_components[guard.look_idx].look_at,
                                            ),
                                    )
                                    .scale(glam::Vec2::splat(
                                        guard.look_components[guard.look_idx].ray_scales[ray_idx],
                                    )),
                            )
                        })
                        .count();
                })
                .count();

            // Draw rays for compute_move_component
            all_guards
                .iter()
                .map(|guard| {
                    guard
                        .compute_move_component
                        .ray_lines
                        .iter()
                        .map(|ray_line| {
                            sprite_component::render_mesh(
                                ctx,
                                quad_ctx,
                                ray_line,
                                DrawParam::default()
                                    .dest(self.camera.world_position(guard.transform.position))
                                    .rotation(
                                        -constants::PI / 2.
                                            - util::get_vec_angle(
                                                guard.look_components[guard.look_idx].look_at,
                                            ),
                                    ),
                            )
                        })
                        .count();
                })
                .count();
        }

        n_objects_drawn += self
            .walls
            .iter()
            .filter(|wall| self.camera.contains(&wall.aabb))
            .map(|wall| {
                sprite_component::render(
                    ctx,
                    quad_ctx,
                    &wall.sprite,
                    DrawParam::default()
                        .dest(self.camera.world_position(wall.transform.position))
                        .scale(wall.sprite.scale)
                        .color(graphics::Color::new(
                            wall.brightness,
                            wall.brightness * self.are_guards_alerted.then(|| 0.5).unwrap_or(1.),
                            wall.brightness * self.are_guards_alerted.then(|| 0.5).unwrap_or(1.),
                            1.,
                        )),
                )
            })
            .count();

        sprite_component::render_mesh(
            ctx,
            quad_ctx,
            &self.player.stamina.stamina_mesh,
            DrawParam::default().scale(glam::vec2(self.player.stamina.get_percentage(), 1.)),
        )
        .unwrap();

        // Draw touch input
        if self.is_touch_joystick_activated {
            sprite_component::render_mesh(
                ctx,
                quad_ctx,
                &self.mouse_input_handler.touch_area,
                DrawParam::default(),
            )
            .unwrap();
            sprite_component::render_mesh(
                ctx,
                quad_ctx,
                &self.mouse_input_handler.direction_circle,
                DrawParam::default().offset(-self.mouse_input_handler.direction_offset),
            )
            .unwrap();
        }

        if self.target.is_dead() {
            sprite_component::render_sprite(
                ctx,
                quad_ctx,
                &self.exit.sprite,
                DrawParam::default()
                    .dest(self.camera.world_position(self.exit.transform.position))
                    .rotation(-self.exit.scale_rotation_counter),
            )?;
        }

        self.particle_system.draw(ctx, quad_ctx, &self.camera)?;

        self.overlay_system.draw(ctx, quad_ctx, &self.camera)?;

        if self.game_state == GameState::GameOver {
            graphics::draw(
                ctx,
                quad_ctx,
                &self.menu_rectangle,
                graphics::DrawParam::default()
                    .dest(glam::vec2(
                        constants::WIDTH as f32 * 0.4,
                        constants::HEIGHT as f32 * 0.4166,
                    ))
                    .color(graphics::Color::BLACK),
            )
            .unwrap();

            graphics::queue_text(
                ctx,
                &util::make_text("Game Over".to_owned(), 32.),
                glam::vec2(
                    constants::WIDTH as f32 * 0.47,
                    constants::HEIGHT as f32 * 0.45,
                ),
                Some(graphics::Color::RED),
            );

            graphics::draw(
                ctx,
                quad_ctx,
                &self.menu_rectangle,
                graphics::DrawParam::default()
                    .dest(constants::BTN_BOTTOM_LEFT_POS)
                    .color(graphics::Color::BLACK),
            )
            .unwrap();

            graphics::queue_text(
                ctx,
                &util::make_text("Menu".to_owned(), 32.),
                constants::BTN_BOTTOM_LEFT_POS
                    + glam::vec2(
                        constants::WIDTH as f32 * 0.1,
                        constants::HEIGHT as f32 * 0.033,
                    ),
                None,
            );

            graphics::draw(
                ctx,
                quad_ctx,
                &self.menu_rectangle,
                graphics::DrawParam::default()
                    .dest(constants::BTN_BOTTOM_RIGHT_POS)
                    .color(graphics::Color::BLACK),
            )
            .unwrap();

            graphics::queue_text(
                ctx,
                &util::make_text("Restart".to_owned(), 32.),
                constants::BTN_BOTTOM_RIGHT_POS
                    + glam::vec2(
                        constants::WIDTH as f32 * 0.09,
                        constants::HEIGHT as f32 * 0.033,
                    ),
                None,
            );
        } else if self.game_state == GameState::Pause {
            graphics::draw(
                ctx,
                quad_ctx,
                &self.menu_rectangle,
                graphics::DrawParam::default()
                    .dest(glam::vec2(
                        constants::WIDTH as f32 * 0.4,
                        constants::HEIGHT as f32 * 0.4166,
                    ))
                    .color(graphics::Color::BLACK),
            )
            .unwrap();
            graphics::draw(
                ctx,
                quad_ctx,
                &self.menu_rectangle,
                graphics::DrawParam::default()
                    .dest(constants::BTN_BOTTOM_LEFT_POS)
                    .color(graphics::Color::BLACK),
            )
            .unwrap();
            graphics::draw(
                ctx,
                quad_ctx,
                &self.menu_rectangle,
                graphics::DrawParam::default()
                    .dest(constants::BTN_BOTTOM_RIGHT_POS)
                    .color(graphics::Color::BLACK),
            )
            .unwrap();

            graphics::queue_text(
                ctx,
                &util::make_text("Pause".to_owned(), 32.),
                glam::vec2(
                    constants::WIDTH as f32 * 0.5,
                    constants::HEIGHT as f32 * 0.45,
                ),
                None,
            );

            graphics::queue_text(
                ctx,
                &util::make_text("Menu".to_owned(), 32.),
                constants::BTN_BOTTOM_LEFT_POS
                    + glam::vec2(
                        constants::WIDTH as f32 * 0.1,
                        constants::HEIGHT as f32 * 0.033,
                    ),
                None,
            );

            graphics::queue_text(
                ctx,
                &util::make_text("Restart".to_owned(), 32.),
                constants::BTN_BOTTOM_RIGHT_POS
                    + glam::vec2(
                        constants::WIDTH as f32 * 0.09,
                        constants::HEIGHT as f32 * 0.033,
                    ),
                None,
            );
        }

        if self.level_idx >= level::TUTORIAL_COUNT {
            graphics::queue_text(
                ctx,
                &util::make_text(format!("{:.1}", self.curr_level_time), 24.),
                glam::vec2(
                    constants::WIDTH as f32 * 0.9,
                    constants::HEIGHT as f32 * 0.95,
                ),
                None,
            );
        }

        if self.debug_draw {
            graphics::draw(
                ctx,
                quad_ctx,
                &util::make_text(format!("{} / {}", n_objects_drawn, self.n_objects), 24.),
                DrawParam::from((glam::vec2(8., 40.),)),
            )?;
            graphics::draw(
                ctx,
                quad_ctx,
                &util::make_text(
                    format!("player grid index: {}", self.player.transform.grid_index),
                    24.,
                ),
                DrawParam::from((glam::vec2(8., 80.),)),
            )?;

            // Draw grid
            for x in (0..=constants::MAX_WORLD_X).step_by(constants::GRID_CELL_SIZE) {
                sprite_component::render_mesh(
                    ctx,
                    quad_ctx,
                    &self.grid_line,
                    DrawParam::default()
                        .dest(self.camera.world_position(glam::vec2(x as f32, 0.)))
                        .rotation(constants::PI / 2.),
                )?;
            }
            for y in (0..=constants::MAX_WORLD_Y).step_by(constants::GRID_CELL_SIZE) {
                sprite_component::render_mesh(
                    ctx,
                    quad_ctx,
                    &self.grid_line,
                    DrawParam::default().dest(self.camera.world_position(glam::vec2(0., y as f32))),
                )?;
            }
        }

        graphics::draw_queued_text(
            ctx,
            quad_ctx,
            graphics::DrawParam::default(),
            None,
            graphics::default_filter(ctx),
        )?;

        Ok(())
    }

    fn write_config(&self, filename: &str) {
        let json = serde_json::json!(
        {
            "mute": !self.sound_collection.is_on,
            "particles": self.particle_system.is_activated,
            "skip_tutorial": self.is_skip_tutorial
        });

        std::fs::write(filename, serde_json::to_string_pretty(&json).unwrap()).unwrap();
    }
}

impl ggez::event::EventHandler<ggez::GameError> for Game {
    fn update(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::Context,
    ) -> Result<(), ggez::GameError> {
        // NOTE: Since this spawns a thread, it is okay to block here
        self.tokio_runtime.block_on(self.network_system.tick());

        if self.game_state == GameState::Menu
            || self.game_state == GameState::Info
            || self.game_state == GameState::Leaderboard
            || self.game_state == GameState::GameOver
            || self.game_state == GameState::Pause
            || self.game_state == GameState::EndScreen
            || self.game_state == GameState::SubmitTime
        {
            return Ok(());
        }

        let dt = ggez::timer::delta(ctx).as_secs_f32();

        if self.game_state == GameState::LevelAnimation {
            self.do_level_animation(dt);

            return Ok(());
        }

        mouse_input_handler::system(self, ggez::timer::time_since_start(ctx).as_secs_f32());

        entities::wall::check_collision(self);

        entities::player::system(ctx, self, dt);

        self.target.update(dt);

        entities::guards::system(ctx, self, dt);

        look_component::system(self);

        entities::exit::system(self, dt);

        level::system(self);

        particle_system::system(self, dt);

        overlay_system::system(self, dt);

        camera_component::system(self);

        self.curr_level_time += dt;

        if self.exit.player_exited {
            self.next_level(ctx, quad_ctx, true);
        }

        Ok(())
    }

    fn draw(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::Context,
    ) -> Result<(), ggez::GameError> {
        graphics::clear(ctx, quad_ctx, constants::BG_COLOR);

        if self.game_state == GameState::Menu {
            return self.draw_menu(ctx, quad_ctx);
        }

        if self.game_state == GameState::Info {
            return self.draw_info(ctx, quad_ctx);
        }

        if self.game_state == GameState::Leaderboard {
            return self.draw_leaderboard(ctx, quad_ctx);
        }

        if self.game_state == GameState::EndScreen {
            return self.draw_end_screen(ctx, quad_ctx);
        }

        self.draw_game(ctx, quad_ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        repeat: bool,
    ) {
        if self.game_state == GameState::EndScreen {
            if keycode == KeyCode::Backspace {
                self.player_name.pop();
            }

            return;
        }

        if (self.game_state == GameState::Game || self.game_state == GameState::Pause)
            && keycode == KeyCode::Escape
        {
            self.game_state = if self.game_state == GameState::Game {
                GameState::Pause
            } else {
                GameState::Game
            };

            return;
        }

        if keycode != KeyCode::F {
            self.double_press_timer = None;
        }

        match keycode {
            KeyCode::W | KeyCode::Up => self.player.set_y_dir(-1.),
            KeyCode::S | KeyCode::Down => self.player.set_y_dir(1.),
            KeyCode::A | KeyCode::Left => self.player.set_x_dir(-1.),
            KeyCode::D | KeyCode::Right => self.player.set_x_dir(1.),
            KeyCode::F => {
                let curr_t = ggez::timer::time_since_start(ctx).as_secs_f32();

                if self.game_state == GameState::GameOver {
                    return;
                }

                if repeat {
                    self.player.set_stealth_intent(true);
                } else if curr_t - self.double_press_timer.unwrap_or(-1.)
                    < constants::DOUBLE_PRESS_TIME
                {
                    self.player.teleport_action(
                        ctx,
                        &mut self.sound_collection,
                        &mut self.particle_system,
                    );
                    self.double_press_timer = None;
                } else {
                    self.double_press_timer = Some(curr_t);
                }
            }
            KeyCode::LeftControl | KeyCode::C => {
                self.player.set_move_type(entities::player::MoveType::Slow)
            }
            KeyCode::LeftShift => self
                .player
                .set_move_type(entities::player::MoveType::Sprint),
            KeyCode::Q | KeyCode::E => entities::player::try_attack_guard(ctx, self),
            KeyCode::M => self.sound_collection.is_on = !self.sound_collection.is_on,
            KeyCode::R => level::load_level(ctx, quad_ctx, self, self.level_idx, false),
            KeyCode::B => self.debug_draw = !self.debug_draw,
            KeyCode::L => self.next_level(ctx, quad_ctx, true), // TODO: Delete this [debugging purposes]
            KeyCode::K => self.player.transform.set(self.target.transform.position), // TODO: Delete this [debugging purposes]
            KeyCode::P => entities::guards::alert_all(ctx, self), // TODO: Delete this [debugging purposes]
            _ => (),
        };
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        _quad_ctx: &mut miniquad::Context,
        keycode: KeyCode,
        _keymods: KeyMods,
    ) {
        if (keycode == KeyCode::W || keycode == KeyCode::Up)
            && self.player.move_component.direction.y < 0.
        {
            self.player.set_y_dir(0.);
        } else if (keycode == KeyCode::S || keycode == KeyCode::Down)
            && self.player.move_component.direction.y > 0.
        {
            self.player.set_y_dir(0.);
        } else if (keycode == KeyCode::A || keycode == KeyCode::Left)
            && self.player.move_component.direction.x < 0.
        {
            self.player.set_x_dir(0.);
        } else if (keycode == KeyCode::D || keycode == KeyCode::Right)
            && self.player.move_component.direction.x > 0.
        {
            self.player.set_x_dir(0.);
        } else if keycode == KeyCode::F {
            self.player.set_stealth_intent(false);
        } else if keycode == KeyCode::LeftControl
            || keycode == KeyCode::C
            || keycode == KeyCode::LeftShift
        {
            self.player
                .set_move_type(entities::player::MoveType::Normal);
        }
    }

    fn text_input_event(
        &mut self,
        _ctx: &mut Context,
        _quad_ctx: &mut miniquad::Context,
        character: char,
    ) {
        if self.game_state == GameState::EndScreen && self.player_name.len() < 28 {
            self.player_name.push(character);
        }
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::GraphicsContext,
        button: MouseButton,
        x: f32,
        y: f32,
    ) {
        match button {
            MouseButton::Left => {
                let curr_t = ggez::timer::time_since_start(ctx).as_secs_f32();

                if self.game_state == GameState::Menu {
                    let mut setting_changed = false;

                    if util::rect_contains_point(
                        constants::BTN_DIM_SQUARE,
                        constants::BTN_BOTTOM_LEFT_POS1,
                        glam::vec2(x, y),
                    ) {
                        self.sound_collection.is_on = !self.sound_collection.is_on;

                        setting_changed = true;
                    } else if util::rect_contains_point(
                        constants::BTN_DIM_SQUARE,
                        constants::BTN_BOTTOM_LEFT_POS2,
                        glam::vec2(x, y),
                    ) {
                        self.particle_system
                            .set_activated(!self.particle_system.is_activated);

                        setting_changed = true;
                    } else if util::rect_contains_point(
                        constants::BTN_DIM_SQUARE,
                        constants::BTN_BOTTOM_LEFT_POS3,
                        glam::vec2(x, y),
                    ) {
                        self.is_skip_tutorial = !self.is_skip_tutorial;

                        setting_changed = true;
                    }

                    if setting_changed {
                        self.write_config(&util::config_filename());
                    }
                }

                if self.game_state == GameState::Menu
                    || self.game_state == GameState::Info
                    || self.game_state == GameState::Leaderboard
                    || self.game_state == GameState::GameOver
                    || self.game_state == GameState::EndScreen
                    || self.game_state == GameState::Pause
                {
                    if let Some(new_game_state) =
                        self.mouse_input_handler
                            .handle_menu_pressed(&self.game_state, x, y)
                    {
                        if new_game_state == GameState::Game {
                            let is_proceed = self.game_state != GameState::GameOver
                                && self.game_state != GameState::Pause;

                            if self.game_state == GameState::Menu {
                                self.level_idx = 0;
                                if self.is_skip_tutorial {
                                    self.level_idx = level::TUTORIAL_COUNT;
                                }
                            }

                            level::load_level(ctx, quad_ctx, self, self.level_idx, is_proceed);
                            self.n_objects = 1
                                + self.guards_basic.len()
                                + self.guards_scout.len()
                                + self.walls.len();

                            if is_proceed {
                                self.game_state = GameState::LevelAnimation;
                                self.curr_level_time = constants::LEVEL_ANIMATION_TIME;
                            } else {
                                self.game_state = GameState::Game
                            }
                        } else {
                            self.game_state = new_game_state;
                            self.curr_level_time = 0.;

                            if self.game_state == GameState::Menu {
                                self.level_idx = 0;
                            }
                        }

                        if self.game_state == GameState::Leaderboard
                            && !self.network_system.request_in_progress()
                        {
                            self.network_system.do_request_leaderboard();
                        }

                        if self.game_state == GameState::SubmitTime {
                            self.leaderboard_str = None;

                            let total_time = self
                                .level_times
                                .iter()
                                .skip(level::TUTORIAL_COUNT)
                                .sum::<f32>();
                            self.network_system.do_submit_time_and_reqeust_leaderboard(
                                self.player_name.clone(),
                                total_time,
                            );

                            self.player_name = String::new();
                            self.game_state = GameState::Leaderboard;
                        }
                    }

                    return;
                }

                // NOTE: there is no PlayerAction to handle here
                self.mouse_input_handler.handle_game_pressed(true, curr_t);
            }
            _ => (),
        }
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        _quad_ctx: &mut miniquad::Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        match button {
            MouseButton::Left => {
                let curr_t = ggez::timer::time_since_start(ctx).as_secs_f32();

                match self.mouse_input_handler.handle_game_pressed(false, curr_t) {
                    Some(mouse_input_handler::PlayerAction::Teleport) => {
                        self.player.teleport_action(
                            ctx,
                            &mut self.sound_collection,
                            &mut self.particle_system,
                        )
                    }
                    Some(mouse_input_handler::PlayerAction::Stealth(is_stealth)) => {
                        self.player.set_stealth_intent(is_stealth)
                    }
                    Some(mouse_input_handler::PlayerAction::StopMoving) => {
                        self.player.set_dir(glam::Vec2::ZERO)
                    }
                    None => (),
                }
            }
            _ => (),
        }
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _quad_ctx: &mut miniquad::Context,
        x: f32,
        y: f32,
        _dx: f32,
        _dy: f32,
    ) {
        let direction = self
            .mouse_input_handler
            .get_move_direction(glam::vec2(x, y));

        if let Some(dir) = direction {
            self.player.set_dir(dir);
        }

        self.is_touch_joystick_activated = direction.is_some();
        if direction.is_some() {
            self.player.set_stealth_intent(false);
        }
    }
}

fn main() -> GameResult {
    let conf = ggez::conf::Conf::default()
        .window_title("Ultimate Ninja".to_owned())
        .window_width(constants::WIDTH as i32)
        .window_height(constants::HEIGHT as i32)
        .cache(Some(include_bytes!("resources.tar")));

    ggez::start(conf, |mut context, mut quad_ctx| {
        Box::new(Game::new(&mut context, &mut quad_ctx))
    })
}
