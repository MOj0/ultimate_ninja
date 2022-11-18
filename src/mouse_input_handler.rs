use crate::constants;
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

pub enum PlayerAblity {
    Teleport,
    Stealth(bool),
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
            direction_offset: glam::Vec2::default(),
            draw_scale,
            pressed_time: None,
            hold_time: None,
        }
    }

    pub fn handle_pressed(&mut self, is_pressed: bool, curr_time: f32) -> Option<PlayerAblity> {
        let mut player_ablity: Option<PlayerAblity> = None;

        // Touch is released
        if self.is_pressed && !is_pressed {
            if curr_time - self.hold_time.unwrap_or(curr_time) > constants::HOLD_THRESHOLD_TIME {
                player_ablity = Some(PlayerAblity::Stealth(false));
                self.hold_time = None;
            } else if curr_time - self.pressed_time.unwrap_or(-1.) < constants::DOUBLE_PRESS_TIME {
                player_ablity = Some(PlayerAblity::Teleport);
                self.pressed_time = None;
            } else {
                self.pressed_time = Some(curr_time);
            }

            self.direction_offset = glam::Vec2::default();
        }

        if is_pressed {
            self.hold_time = Some(curr_time);
        }

        self.is_pressed = is_pressed;
        return player_ablity;
    }

    pub fn get_player_action(&mut self, curr_time: f32) -> Option<PlayerAblity> {
        if self.is_pressed
            && curr_time - self.hold_time.unwrap_or(curr_time) > constants::HOLD_THRESHOLD_TIME
        {
            return Some(PlayerAblity::Stealth(true));
        }

        None
    }

    pub fn get_move_direction(&mut self, mouse_position: glam::Vec2) -> Option<glam::Vec2> {
        if !self.is_pressed {
            return None;
        }

        self.pressed_time = None;
        self.hold_time = None;

        let diff = mouse_position - self.center;
        let diff_len = diff.length();
        let diff_normalized = diff.normalize_or_zero();

        self.direction_offset = diff_normalized * self.draw_scale.min(diff_len);

        (diff_len > self.draw_scale)
            .then_some(diff_normalized)
            .or(Some(diff_normalized * diff_len / self.draw_scale))
    }
}

pub fn system(game_state: &mut GameState, curr_time: f32) {
    if let Some(PlayerAblity::Stealth(is_stealth)) =
        game_state.mouse_input_handler.get_player_action(curr_time)
    {
        game_state.player.set_stealth_intent(is_stealth);
    }
}
