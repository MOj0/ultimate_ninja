use crate::look_component::LookComponent;
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
