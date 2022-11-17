use crate::constants;
use crate::GameState;

pub struct MouseInputHandler {
    pub center: glam::Vec2,
    pub is_pressed: bool,
    pub circle_mesh: ggez::graphics::Mesh,
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
    ) -> Self {
        let circle_mesh = ggez::graphics::Mesh::new_circle(
            ctx,
            quad_ctx,
            ggez::graphics::DrawMode::fill(),
            center,
            20.,
            0.1,
            ggez::graphics::Color::BLUE,
        )
        .unwrap();

        Self {
            center,
            is_pressed: false,
            circle_mesh,
            pressed_time: None,
            hold_time: None,
        }
    }

    pub fn handle_pressed(&mut self, is_pressed: bool, curr_time: f32) -> Option<PlayerAblity> {
        let mut player_ablity: Option<PlayerAblity> = None;

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
        let diff_normalized = diff.normalize_or_zero();

        (diff.length_squared() < 1500.)
            .then_some(diff_normalized / 2.)
            .or_else(|| Some(diff_normalized))
    }
}

pub fn system(game_state: &mut GameState, curr_time: f32) {
    if let Some(PlayerAblity::Stealth(is_stealth)) =
        game_state.mouse_input_handler.get_player_action(curr_time)
    {
        game_state.player.set_stealth_intent(is_stealth);
    }
}
