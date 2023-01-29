use crate::constants;
use crate::util;
use crate::Game;
use crate::GameState;

pub struct MouseInputHandler {
    pub center: glam::Vec2,
    pub is_pressed: bool,
    pub touch_area: ggez::graphics::Mesh,
    pub direction_circle: ggez::graphics::Mesh,
    pub direction_offset: glam::Vec2,
    pub draw_scale: f32,
    pressed_time: Option<f32>,
    hold_time: Option<f32>,
}

pub enum PlayerAction {
    Teleport,
    Stealth(bool),
    StopMoving,
}

impl MouseInputHandler {
    pub fn new(
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::miniquad::GraphicsContext,
        center: glam::Vec2,
        draw_scale: f32,
    ) -> Self {
        let touch_area = ggez::graphics::Mesh::new_circle(
            ctx,
            quad_ctx,
            ggez::graphics::DrawMode::fill(),
            center,
            draw_scale,
            0.1,
            ggez::graphics::Color::new(0., 0., 0., 0.65),
        )
        .unwrap();

        let direction_circle = ggez::graphics::Mesh::new_circle(
            ctx,
            quad_ctx,
            ggez::graphics::DrawMode::fill(),
            center,
            10.,
            0.1,
            ggez::graphics::Color::BLUE,
        )
        .unwrap();

        Self {
            center,
            is_pressed: false,
            touch_area,
            direction_circle,
            direction_offset: glam::Vec2::ZERO,
            draw_scale,
            pressed_time: None,
            hold_time: None,
        }
    }

    pub fn handle_menu_pressed(
        &self,
        game_state: &GameState,
        screen_size: (f32, f32),
        mx: f32,
        my: f32,
    ) -> Option<GameState> {
        let mouse_vec = glam::vec2(mx, my);

        if *game_state == GameState::Menu {
            if util::rect_contains_point(
                screen_size,
                constants::BTN_DIM_RECT,
                constants::BTN_PLAY_POS,
                mouse_vec,
            ) {
                return Some(GameState::Game);
            }

            if util::rect_contains_point(
                screen_size,
                constants::BTN_DIM_RECT,
                constants::BTN_INFO_POS,
                mouse_vec,
            ) {
                return Some(GameState::Info);
            }

            if util::rect_contains_point(
                screen_size,
                constants::BTN_DIM_RECT,
                constants::BTN_BOTTOM_RIGHT_POS,
                mouse_vec,
            ) {
                return Some(GameState::Leaderboard);
            }
        }

        if *game_state == GameState::Info || *game_state == GameState::Leaderboard {
            if util::rect_contains_point(
                screen_size,
                constants::BTN_DIM_RECT,
                constants::BTN_BACK_POS,
                mouse_vec,
            ) {
                return Some(GameState::Menu);
            }
        }

        if *game_state == GameState::GameOver || *game_state == GameState::Pause {
            if util::rect_contains_point(
                screen_size,
                constants::BTN_DIM_RECT,
                constants::BTN_BOTTOM_LEFT_POS,
                mouse_vec,
            ) {
                return Some(GameState::Menu);
            } else if util::rect_contains_point(
                screen_size,
                constants::BTN_DIM_RECT,
                constants::BTN_BOTTOM_RIGHT_POS,
                mouse_vec,
            ) {
                return Some(GameState::Game);
            }
        }

        if *game_state == GameState::EndScreen {
            if util::rect_contains_point(
                screen_size,
                constants::BTN_DIM_RECT,
                constants::BTN_BOTTOM_RIGHT_POS,
                mouse_vec,
            ) {
                return Some(GameState::SubmitTime);
            }

            if util::rect_contains_point(
                screen_size,
                constants::BTN_DIM_RECT,
                constants::BTN_BOTTOM_LEFT_POS,
                mouse_vec,
            ) {
                return Some(GameState::Menu);
            }
        }

        return None;
    }

    pub fn handle_game_pressed(
        &mut self,
        is_pressed: bool,
        curr_time: f32,
    ) -> Option<PlayerAction> {
        let mut player_action: Option<PlayerAction> = None;

        // Touch is released
        if self.is_pressed && !is_pressed {
            if curr_time - self.hold_time.unwrap_or(curr_time) > constants::HOLD_THRESHOLD_TIME {
                player_action = Some(PlayerAction::Stealth(false));
                self.hold_time = None;
            } else if curr_time - self.pressed_time.unwrap_or(-1.) < constants::DOUBLE_PRESS_TIME {
                player_action = Some(PlayerAction::Teleport);
                self.pressed_time = None;
            } else {
                player_action = Some(PlayerAction::StopMoving);
                self.pressed_time = Some(curr_time);
            }

            self.direction_offset = glam::Vec2::ZERO;
        }

        if is_pressed {
            self.hold_time = Some(curr_time);
        }

        self.is_pressed = is_pressed;
        return player_action;
    }

    pub fn get_player_action(&mut self, curr_time: f32) -> Option<PlayerAction> {
        if self.is_pressed
            && curr_time - self.hold_time.unwrap_or(curr_time) > constants::HOLD_THRESHOLD_TIME
        {
            return Some(PlayerAction::Stealth(true));
        }

        None
    }

    pub fn get_move_direction(
        &mut self,
        screen_size: (f32, f32),
        mouse_position: glam::Vec2,
    ) -> Option<glam::Vec2> {
        if !self.is_pressed {
            return None;
        }

        self.pressed_time = None;
        self.hold_time = None;

        let scale = glam::vec2(
            screen_size.0 / constants::WIDTH as f32,
            screen_size.1 / constants::HEIGHT as f32,
        );

        let diff = mouse_position - self.center * scale;
        let diff_len = diff.length();
        let diff_normalized = diff.normalize_or_zero();

        let sc_x = constants::WIDTH as f32 / screen_size.0;
        let sc_y = constants::HEIGHT as f32 / screen_size.1;

        self.direction_offset = glam::vec2(
            diff_normalized.x * (diff_len * sc_x).min(self.draw_scale),
            diff_normalized.y * (diff_len * sc_y).min(self.draw_scale),
        );

        (diff_len > self.draw_scale)
            .then(|| diff_normalized)
            .or(Some(diff_normalized * diff_len / self.draw_scale))
    }
}

pub fn system(game_state: &mut Game, curr_time: f32) {
    if let Some(PlayerAction::Stealth(is_stealth)) =
        game_state.mouse_input_handler.get_player_action(curr_time)
    {
        game_state.player.set_stealth_intent(is_stealth);
    }
}
