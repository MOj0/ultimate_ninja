use crate::constants;
use crate::sprite_component::SpriteComponent;
use crate::transform_component::TransformComponent;
use crate::util;

pub struct LookComponent {
    pub look_at: glam::Vec2,
    pub fov: f32,
    pub view_distance: f32,
    pub sprite: SpriteComponent,
}

impl LookComponent {
    pub fn new(look_at: glam::Vec2, fov: f32, view_distance: f32) -> Self {
        Self {
            look_at,
            fov,
            view_distance,
            sprite: SpriteComponent::new(),
        }
    }

    pub fn make_look_polygon(&mut self, transform: &TransformComponent) {
        let line_of_sight = transform.position + self.look_at * self.view_distance;
        let fov_corner1 =
            util::rotate_point_around_center(line_of_sight, transform.position, -self.fov);
        let fov_corner2 =
            util::rotate_point_around_center(line_of_sight, transform.position, self.fov);
        let intersection = (fov_corner1 - fov_corner2) / 2. + fov_corner2;

        let arc_points = util::get_arc_points(
            intersection,
            glam::vec2(
                ((fov_corner1 - fov_corner2) / 2.).length(),
                (line_of_sight - intersection).length(),
            ),
            0.,
            constants::PI,
            -constants::PI / 2. - self.look_at.angle_between(glam::vec2(1., 0.)),
        );

        let fov_points = vec![transform.position, fov_corner1]
            .into_iter()
            .chain(arc_points)
            .chain(std::iter::once(fov_corner2))
            .collect::<Vec<glam::Vec2>>();

        self.sprite.new_polygon(
            ggez::graphics::DrawMode::fill(),
            &fov_points,
            ggez::graphics::Color {
                r: 0.5,
                g: 0.,
                b: 0.,
                a: 0.5,
            },
        );
    }
}
