use crate::constants;
use crate::util;

pub struct LookComponent {
    pub look_at: glam::Vec2,
    pub fov: f32,
    pub view_distance: f32,
    pub fov_mesh: ggez::graphics::Mesh,
}

impl LookComponent {
    pub fn new(
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::miniquad::GraphicsContext,
        look_at: glam::Vec2,
        fov: f32,
        view_distance: f32,
    ) -> Self {
        let fov_points = Self::make_fov_points(look_at, fov, view_distance);
        let fov_mesh = ggez::graphics::Mesh::new_polygon(
            ctx,
            quad_ctx,
            ggez::graphics::DrawMode::fill(),
            &fov_points,
            ggez::graphics::Color::from_rgba(127, 0, 0, 127),
        )
        .unwrap();

        Self {
            look_at,
            fov,
            view_distance,
            fov_mesh,
        }
    }

    fn make_fov_points(look_at: glam::Vec2, fov: f32, view_distance: f32) -> Vec<glam::Vec2> {
        let top_left = glam::vec2(0., 0.);

        let line_of_sight = top_left + look_at * view_distance;
        let fov_corner1 = util::rotate_point_around_center(line_of_sight, top_left, -fov);
        let fov_corner2 = util::rotate_point_around_center(line_of_sight, top_left, fov);
        let intersection = (fov_corner1 - fov_corner2) / 2. + fov_corner2;

        let arc_points = util::get_arc_points(
            intersection,
            glam::vec2(
                ((fov_corner1 - fov_corner2) / 2.).length(),
                (line_of_sight - intersection).length(),
            ),
            0.,
            constants::PI,
            -constants::PI / 2. - util::get_vec_angle(look_at),
        );

        let fov_points = vec![top_left, fov_corner1]
            .into_iter()
            .chain(arc_points)
            .chain(std::iter::once(fov_corner2))
            .collect::<Vec<glam::Vec2>>();

        return fov_points;
    }
}
