use std::fmt;

use crate::animation_component::AnimationComponent;
use crate::assets::Assets;
use crate::constants;
use crate::look_component::LookComponent;
use crate::sprite_component::SpriteComponent;
use crate::transform_component::TransformComponent;

pub fn make_text(s: String, scale: f32) -> ggez::graphics::Text {
    ggez::graphics::Text::new(ggez::graphics::TextFragment::new(s).scale(scale))
}

pub fn rotate_point_around_center(point: glam::Vec2, center: glam::Vec2, angle: f32) -> glam::Vec2 {
    let r_x = angle.cos() * (point.x - center.x) - angle.sin() * (point.y - center.y) + center.x;
    let r_y = angle.sin() * (point.x - center.x) + angle.cos() * (point.y - center.y) + center.y;

    glam::vec2(r_x, r_y)
}

#[inline]
pub fn check_collision(t1: &TransformComponent, t2: &TransformComponent) -> bool {
    let distance_vec = t1.position - t2.position;
    distance_vec.length_squared() <= (t1.size + t2.size).powi(2)
}

pub fn check_spotted(
    look: &LookComponent,
    source: &TransformComponent,
    dest: &TransformComponent,
    rect_objects: &Vec<&ggez::graphics::Rect>,
) -> bool {
    let vec_to_dest = dest.position - source.position;
    let len_to_dest_sq = vec_to_dest.length_squared();

    // Source and dest are colliding
    if len_to_dest_sq <= (source.size + dest.size).powi(2) {
        return true;
    }
    // Dest is out of view distance
    if len_to_dest_sq > look.view_distance.powi(2) {
        return false;
    }

    // Check if there is an collision object between the source and destination
    let line_to_dest = (
        source.position.x,
        source.position.y,
        dest.position.x,
        dest.position.y,
    );
    let line_of_sight_blocked = rect_objects.iter().any(|rect| {
        line_rect_intersects(
            line_to_dest.0,
            line_to_dest.1,
            line_to_dest.2,
            line_to_dest.3,
            rect,
        )
    });
    if line_of_sight_blocked {
        return false;
    }

    // Check spotted
    let dot = look.look_at.dot(vec_to_dest);
    let angle = (dot / (look.look_at.length() * len_to_dest_sq.sqrt())).acos();
    return angle <= look.fov;
}

pub fn line_line_intersects(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
    x4: f32,
    y4: f32,
) -> bool {
    // calculate the direction of the lines
    let u_a = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3))
        / ((y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1));
    let u_b = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3))
        / ((y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1));

    // if u_a and u_b are between 0-1, lines are colliding
    u_a >= 0. && u_a <= 1. && u_b >= 0. && u_b <= 1.
}
pub fn line_line_intersection(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
    x4: f32,
    y4: f32,
) -> Option<glam::Vec2> {
    if !line_line_intersects(x1, y1, x2, y2, x3, y3, x4, y4) {
        return None;
    }

    //Line1
    let a1 = y2 - y1;
    let b1 = x1 - x2;
    let c1 = a1 * x1 + b1 * y1;

    //Line2
    let a2 = y4 - y3;
    let b2 = x3 - x4;
    let c2 = a2 * x3 + b2 * y3;

    let det = a1 * b2 - a2 * b1;
    if det.abs() > f32::EPSILON {
        let x = (b2 * c1 - b1 * c2) / det;
        let y = (a1 * c2 - a2 * c1) / det;
        return Some(glam::vec2(x, y));
    }

    None
}

pub fn line_rect_intersects(
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    rect: &ggez::graphics::Rect,
) -> bool {
    let top_intersection = line_line_intersects(
        x0,
        y0,
        x1,
        y1,
        rect.left(),
        rect.top(),
        rect.right(),
        rect.top(),
    );
    let right_intersection = line_line_intersects(
        x0,
        y0,
        x1,
        y1,
        rect.right(),
        rect.top(),
        rect.right(),
        rect.bottom(),
    );
    let bottom_intersection = line_line_intersects(
        x0,
        y0,
        x1,
        y1,
        rect.left(),
        rect.bottom(),
        rect.right(),
        rect.bottom(),
    );
    let left_intersection = line_line_intersects(
        x0,
        y0,
        x1,
        y1,
        rect.left(),
        rect.top(),
        rect.left(),
        rect.bottom(),
    );

    top_intersection || right_intersection || bottom_intersection || left_intersection
}

pub fn line_rect_intersection(
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    rect: &ggez::graphics::Rect,
) -> Option<glam::Vec2> {
    if !line_rect_intersects(x0, y0, x1, y1, rect) {
        return None;
    }

    let top_intersection = line_line_intersection(
        x0,
        y0,
        x1,
        y1,
        rect.left(),
        rect.top(),
        rect.right(),
        rect.top(),
    );
    let right_intersection = line_line_intersection(
        x0,
        y0,
        x1,
        y1,
        rect.right(),
        rect.top(),
        rect.right(),
        rect.bottom(),
    );

    let bottom_intersection = line_line_intersection(
        x0,
        y0,
        x1,
        y1,
        rect.left(),
        rect.bottom(),
        rect.right(),
        rect.bottom(),
    );

    let left_intersection = line_line_intersection(
        x0,
        y0,
        x1,
        y1,
        rect.left(),
        rect.top(),
        rect.left(),
        rect.bottom(),
    );

    vec![
        top_intersection,
        left_intersection,
        right_intersection,
        bottom_intersection,
    ]
    .into_iter()
    .filter_map(|intersection| intersection)
    .min_by_key(|intersect_point| {
        ((x0 - intersect_point.x).abs() + (y0 - intersect_point.y).abs()) as u32
    })
}

// NOTE: y is inverted because screen's coordinate system
pub fn vec_from_angle(angle: f32) -> glam::Vec2 {
    glam::vec2(angle.cos(), -angle.sin())
}

pub fn get_vec_angle(v: glam::Vec2) -> f32 {
    if v.length_squared() == 0. {
        return 0.;
    }
    v.angle_between(glam::vec2(1., 0.))
}

pub fn lerp(from: f32, to: f32, delta: f32) -> f32 {
    from + delta * (to - from)
}

pub fn vec_lerp(from: glam::Vec2, to: glam::Vec2, delta: f32) -> glam::Vec2 {
    let lerp_x = lerp(from.x, to.x, delta);
    let lerp_y = lerp(from.y, to.y, delta);

    glam::vec2(lerp_x, lerp_y)
}

pub fn make_particle_image(
    ctx: &mut ggez::Context,
    quad_ctx: &mut ggez::miniquad::Context,
) -> ggez::graphics::Image {
    let bytes = [u8::MAX; 4]; // 1 pixel texture with 1.0 in every color

    ggez::graphics::Image::from_rgba8(ctx, quad_ctx, 1, 1, &bytes).unwrap()
}

pub fn build_walk_animation(
    assets: &Assets,
    duration: f32,
    color: ggez::graphics::Color,
) -> AnimationComponent {
    let s1 = SpriteComponent::new(assets.stand.clone(), color).scale(constants::SPRITE_SCALE);
    let s2 = SpriteComponent::new(assets.walk_l1.clone(), color).scale(constants::SPRITE_SCALE);
    let s3 = SpriteComponent::new(assets.walk_l2.clone(), color).scale(constants::SPRITE_SCALE);
    let s4 = SpriteComponent::new(assets.walk_l3.clone(), color).scale(constants::SPRITE_SCALE);
    let s5 = SpriteComponent::new(assets.walk_l4.clone(), color).scale(constants::SPRITE_SCALE);
    let s6 = SpriteComponent::new(assets.walk_l3.clone(), color).scale(constants::SPRITE_SCALE);
    let s7 = SpriteComponent::new(assets.walk_l2.clone(), color).scale(constants::SPRITE_SCALE);
    let s8 = SpriteComponent::new(assets.walk_l1.clone(), color).scale(constants::SPRITE_SCALE);
    let s9 = SpriteComponent::new(assets.stand.clone(), color).scale(constants::SPRITE_SCALE);
    let s10 = SpriteComponent::new(assets.walk_r1.clone(), color).scale(constants::SPRITE_SCALE);
    let s11 = SpriteComponent::new(assets.walk_r2.clone(), color).scale(constants::SPRITE_SCALE);
    let s12 = SpriteComponent::new(assets.walk_r3.clone(), color).scale(constants::SPRITE_SCALE);
    let s13 = SpriteComponent::new(assets.walk_r4.clone(), color).scale(constants::SPRITE_SCALE);
    let s14 = SpriteComponent::new(assets.walk_r3.clone(), color).scale(constants::SPRITE_SCALE);
    let s15 = SpriteComponent::new(assets.walk_r2.clone(), color).scale(constants::SPRITE_SCALE);
    let s16 = SpriteComponent::new(assets.walk_r1.clone(), color).scale(constants::SPRITE_SCALE);

    let animation = vec![
        s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, s11, s12, s13, s14, s15, s16,
    ];

    AnimationComponent::new(animation, duration)
}

#[inline]
pub fn compute_animation_duration(speed: f32) -> f32 {
    speed.recip() * constants::ANIMATION_SPEED
}

pub fn rect_contains_point(
    screen_size: (f32, f32),
    rect_dim: glam::Vec2,
    rect_pos: glam::Vec2,
    point: glam::Vec2,
) -> bool {
    let scale = glam::vec2(
        screen_size.0 / constants::WIDTH as f32,
        screen_size.1 / constants::HEIGHT as f32,
    );

    let rect_dim = rect_dim * scale;
    let rect_pos = rect_pos * scale;

    point.x >= rect_pos.x
        && point.x <= rect_pos.x + rect_dim.x
        && point.y >= rect_pos.y
        && point.y <= rect_pos.y + rect_dim.y
}

#[inline]
pub fn clamp(v: f32, min: f32, max: f32) -> f32 {
    v.min(max).max(min)
}

pub fn compute_grid_index(position: &glam::Vec2) -> isize {
    let x = position.x as usize / constants::GRID_CELL_SIZE;
    let y = position.y as usize / constants::GRID_CELL_SIZE;

    (x + y * constants::MAX_WORLD_X as usize / constants::GRID_CELL_SIZE) as isize
}

pub struct MyKeyCode(pub ggez::event::KeyCode);

impl From<u64> for MyKeyCode {
    fn from(char: u64) -> Self {
        match char {
            0 => MyKeyCode(ggez::event::KeyCode::Space),
            1 => MyKeyCode(ggez::event::KeyCode::Apostrophe),
            2 => MyKeyCode(ggez::event::KeyCode::Comma),
            3 => MyKeyCode(ggez::event::KeyCode::Minus),
            4 => MyKeyCode(ggez::event::KeyCode::Period),
            5 => MyKeyCode(ggez::event::KeyCode::Slash),
            6 => MyKeyCode(ggez::event::KeyCode::Key0),
            7 => MyKeyCode(ggez::event::KeyCode::Key1),
            8 => MyKeyCode(ggez::event::KeyCode::Key2),
            9 => MyKeyCode(ggez::event::KeyCode::Key3),
            10 => MyKeyCode(ggez::event::KeyCode::Key4),
            11 => MyKeyCode(ggez::event::KeyCode::Key5),
            12 => MyKeyCode(ggez::event::KeyCode::Key6),
            13 => MyKeyCode(ggez::event::KeyCode::Key7),
            14 => MyKeyCode(ggez::event::KeyCode::Key8),
            15 => MyKeyCode(ggez::event::KeyCode::Key9),
            16 => MyKeyCode(ggez::event::KeyCode::Semicolon),
            17 => MyKeyCode(ggez::event::KeyCode::Equal),
            18 => MyKeyCode(ggez::event::KeyCode::A),
            19 => MyKeyCode(ggez::event::KeyCode::B),
            20 => MyKeyCode(ggez::event::KeyCode::C),
            21 => MyKeyCode(ggez::event::KeyCode::D),
            22 => MyKeyCode(ggez::event::KeyCode::E),
            23 => MyKeyCode(ggez::event::KeyCode::F),
            24 => MyKeyCode(ggez::event::KeyCode::G),
            25 => MyKeyCode(ggez::event::KeyCode::H),
            26 => MyKeyCode(ggez::event::KeyCode::I),
            27 => MyKeyCode(ggez::event::KeyCode::J),
            28 => MyKeyCode(ggez::event::KeyCode::K),
            29 => MyKeyCode(ggez::event::KeyCode::L),
            30 => MyKeyCode(ggez::event::KeyCode::M),
            31 => MyKeyCode(ggez::event::KeyCode::N),
            32 => MyKeyCode(ggez::event::KeyCode::O),
            33 => MyKeyCode(ggez::event::KeyCode::P),
            34 => MyKeyCode(ggez::event::KeyCode::Q),
            35 => MyKeyCode(ggez::event::KeyCode::R),
            36 => MyKeyCode(ggez::event::KeyCode::S),
            37 => MyKeyCode(ggez::event::KeyCode::T),
            38 => MyKeyCode(ggez::event::KeyCode::U),
            39 => MyKeyCode(ggez::event::KeyCode::V),
            40 => MyKeyCode(ggez::event::KeyCode::W),
            41 => MyKeyCode(ggez::event::KeyCode::X),
            42 => MyKeyCode(ggez::event::KeyCode::Y),
            43 => MyKeyCode(ggez::event::KeyCode::Z),
            44 => MyKeyCode(ggez::event::KeyCode::LeftBracket),
            45 => MyKeyCode(ggez::event::KeyCode::Backslash),
            46 => MyKeyCode(ggez::event::KeyCode::RightBracket),
            47 => MyKeyCode(ggez::event::KeyCode::GraveAccent),
            48 => MyKeyCode(ggez::event::KeyCode::World1),
            49 => MyKeyCode(ggez::event::KeyCode::World2),
            50 => MyKeyCode(ggez::event::KeyCode::Escape),
            51 => MyKeyCode(ggez::event::KeyCode::Enter),
            52 => MyKeyCode(ggez::event::KeyCode::Tab),
            53 => MyKeyCode(ggez::event::KeyCode::Backspace),
            54 => MyKeyCode(ggez::event::KeyCode::Insert),
            55 => MyKeyCode(ggez::event::KeyCode::Delete),
            56 => MyKeyCode(ggez::event::KeyCode::Right),
            57 => MyKeyCode(ggez::event::KeyCode::Left),
            58 => MyKeyCode(ggez::event::KeyCode::Down),
            59 => MyKeyCode(ggez::event::KeyCode::Up),
            60 => MyKeyCode(ggez::event::KeyCode::PageUp),
            61 => MyKeyCode(ggez::event::KeyCode::PageDown),
            62 => MyKeyCode(ggez::event::KeyCode::Home),
            63 => MyKeyCode(ggez::event::KeyCode::End),
            64 => MyKeyCode(ggez::event::KeyCode::CapsLock),
            65 => MyKeyCode(ggez::event::KeyCode::ScrollLock),
            66 => MyKeyCode(ggez::event::KeyCode::NumLock),
            67 => MyKeyCode(ggez::event::KeyCode::PrintScreen),
            68 => MyKeyCode(ggez::event::KeyCode::Pause),
            69 => MyKeyCode(ggez::event::KeyCode::F1),
            70 => MyKeyCode(ggez::event::KeyCode::F2),
            71 => MyKeyCode(ggez::event::KeyCode::F3),
            72 => MyKeyCode(ggez::event::KeyCode::F4),
            73 => MyKeyCode(ggez::event::KeyCode::F5),
            74 => MyKeyCode(ggez::event::KeyCode::F6),
            75 => MyKeyCode(ggez::event::KeyCode::F7),
            76 => MyKeyCode(ggez::event::KeyCode::F8),
            77 => MyKeyCode(ggez::event::KeyCode::F9),
            78 => MyKeyCode(ggez::event::KeyCode::F10),
            79 => MyKeyCode(ggez::event::KeyCode::F11),
            80 => MyKeyCode(ggez::event::KeyCode::F12),
            81 => MyKeyCode(ggez::event::KeyCode::F13),
            82 => MyKeyCode(ggez::event::KeyCode::F14),
            83 => MyKeyCode(ggez::event::KeyCode::F15),
            84 => MyKeyCode(ggez::event::KeyCode::F16),
            85 => MyKeyCode(ggez::event::KeyCode::F17),
            86 => MyKeyCode(ggez::event::KeyCode::F18),
            87 => MyKeyCode(ggez::event::KeyCode::F19),
            88 => MyKeyCode(ggez::event::KeyCode::F20),
            89 => MyKeyCode(ggez::event::KeyCode::F21),
            90 => MyKeyCode(ggez::event::KeyCode::F22),
            91 => MyKeyCode(ggez::event::KeyCode::F23),
            92 => MyKeyCode(ggez::event::KeyCode::F24),
            93 => MyKeyCode(ggez::event::KeyCode::F25),
            94 => MyKeyCode(ggez::event::KeyCode::Kp0),
            95 => MyKeyCode(ggez::event::KeyCode::Kp1),
            96 => MyKeyCode(ggez::event::KeyCode::Kp2),
            97 => MyKeyCode(ggez::event::KeyCode::Kp3),
            98 => MyKeyCode(ggez::event::KeyCode::Kp4),
            99 => MyKeyCode(ggez::event::KeyCode::Kp5),
            100 => MyKeyCode(ggez::event::KeyCode::Kp6),
            101 => MyKeyCode(ggez::event::KeyCode::Kp7),
            102 => MyKeyCode(ggez::event::KeyCode::Kp8),
            103 => MyKeyCode(ggez::event::KeyCode::Kp9),
            104 => MyKeyCode(ggez::event::KeyCode::KpDecimal),
            105 => MyKeyCode(ggez::event::KeyCode::KpDivide),
            106 => MyKeyCode(ggez::event::KeyCode::KpMultiply),
            107 => MyKeyCode(ggez::event::KeyCode::KpSubtract),
            108 => MyKeyCode(ggez::event::KeyCode::KpAdd),
            109 => MyKeyCode(ggez::event::KeyCode::KpEnter),
            110 => MyKeyCode(ggez::event::KeyCode::KpEqual),
            111 => MyKeyCode(ggez::event::KeyCode::LeftShift),
            112 => MyKeyCode(ggez::event::KeyCode::LeftControl),
            113 => MyKeyCode(ggez::event::KeyCode::LeftAlt),
            114 => MyKeyCode(ggez::event::KeyCode::LeftSuper),
            115 => MyKeyCode(ggez::event::KeyCode::RightShift),
            116 => MyKeyCode(ggez::event::KeyCode::RightControl),
            117 => MyKeyCode(ggez::event::KeyCode::RightAlt),
            118 => MyKeyCode(ggez::event::KeyCode::RightSuper),
            119 => MyKeyCode(ggez::event::KeyCode::Menu),
            _ => MyKeyCode(ggez::event::KeyCode::Unknown),
        }
    }
}

impl fmt::Display for MyKeyCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
