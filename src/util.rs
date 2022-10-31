use crate::animation_component::AnimationComponent;
use crate::assets::Assets;
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

    // Check spotted
    let dot = look.look_at.dot(vec_to_dest);
    let angle = (dot / (look.look_at.length() * len_to_dest_sq.sqrt())).acos();
    return angle <= look.fov;
}

pub fn get_vec_angle(v: glam::Vec2) -> f32 {
    if v.length_squared() == 0. {
        return 0.;
    }
    v.angle_between(glam::vec2(1., 0.))
}

pub fn build_walk_animation(
    assets: &Assets,
    duration: f32,
    color: ggez::graphics::Color,
) -> AnimationComponent {
    let s1 = SpriteComponent::new(assets.stand.clone(), color);
    let s2 = SpriteComponent::new(assets.walk_l1.clone(), color);
    let s3 = SpriteComponent::new(assets.walk_l2.clone(), color);
    let s4 = SpriteComponent::new(assets.walk_l3.clone(), color);
    let s5 = SpriteComponent::new(assets.walk_l4.clone(), color);
    let s6 = SpriteComponent::new(assets.walk_l3.clone(), color);
    let s7 = SpriteComponent::new(assets.walk_l2.clone(), color);
    let s8 = SpriteComponent::new(assets.walk_l1.clone(), color);
    let s9 = SpriteComponent::new(assets.stand.clone(), color);
    let s10 = SpriteComponent::new(assets.walk_r1.clone(), color);
    let s11 = SpriteComponent::new(assets.walk_r2.clone(), color);
    let s12 = SpriteComponent::new(assets.walk_r3.clone(), color);
    let s13 = SpriteComponent::new(assets.walk_r4.clone(), color);
    let s14 = SpriteComponent::new(assets.walk_r3.clone(), color);
    let s15 = SpriteComponent::new(assets.walk_r2.clone(), color);
    let s16 = SpriteComponent::new(assets.walk_r1.clone(), color);

    let animation = vec![
        s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, s11, s12, s13, s14, s15, s16,
    ];

    AnimationComponent::new(animation, duration)
}
