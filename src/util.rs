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

pub fn get_arc_points(
    center: glam::Vec2,
    radii: glam::Vec2,
    start_angle: f32,
    sweep_angle: f32,
    x_rotation: f32,
) -> Vec<glam::Vec2> {
    let arc = lyon_geom::Arc {
        center: lyon_geom::euclid::Point2D {
            x: center.x,
            y: center.y,
            _unit: std::marker::PhantomData,
        },
        radii: lyon_geom::euclid::Vector2D {
            x: radii.x,
            y: radii.y,
            _unit: std::marker::PhantomData,
        },
        start_angle: lyon_geom::Angle::radians(start_angle),
        sweep_angle: lyon_geom::Angle::radians(sweep_angle),
        x_rotation: lyon_geom::Angle::radians(x_rotation),
    };

    arc.flattened(0.01)
        .into_iter()
        .map(|p| glam::vec2(p.x, p.y))
        .collect::<Vec<glam::Vec2>>()
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
    (1.0 - delta) * from + delta * to
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

pub fn rect_contains_point(rect_dim: glam::Vec2, rect_pos: glam::Vec2, point: glam::Vec2) -> bool {
    point.x >= rect_pos.x
        && point.x <= rect_pos.x + rect_dim.x
        && point.y >= rect_pos.y
        && point.y <= rect_pos.y + rect_dim.y
}

#[inline]
pub fn clamp(v: f32, min: f32, max: f32) -> f32 {
    v.min(max).max(min)
}
