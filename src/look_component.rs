use crate::constants;
use crate::transform_component::TransformComponent;
use crate::util;
use crate::GameState;

pub struct LookComponent {
    pub look_at: glam::Vec2,
    pub fov: f32,
    pub view_distance: f32,
    pub fov_mesh: ggez::graphics::Mesh,
    pub rays: Vec<glam::Vec2>,
    pub ray_meshes: Vec<ggez::graphics::Mesh>,
    pub tmp_ray_colors: Vec<ggez::graphics::Color>,
}

impl LookComponent {
    pub fn new(
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::miniquad::GraphicsContext,
        look_at: glam::Vec2,
        fov: f32,
        view_distance: f32,
        n_rays: u32,
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

        let rays = Self::make_rays(look_at, fov, view_distance, n_rays);

        // Build a line mesh for each ray
        let ray_meshes = rays
            .iter()
            .map(|ray| {
                ggez::graphics::Mesh::new_line(
                    ctx,
                    quad_ctx,
                    &[glam::vec2(0., 0.), glam::vec2(ray.x, ray.y)],
                    2.0,
                    ggez::graphics::Color::WHITE,
                )
                .unwrap()
            })
            .collect::<Vec<ggez::graphics::Mesh>>();

        let ray_colors = rays
            .iter()
            .map(|_| ggez::graphics::Color::WHITE)
            .collect::<Vec<ggez::graphics::Color>>();

        Self {
            look_at,
            fov,
            view_distance,
            fov_mesh,
            rays,
            ray_meshes,
            tmp_ray_colors: ray_colors,
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

    fn make_rays(
        look_at: glam::Vec2,
        fov: f32,
        view_distance: f32,
        n_rays: u32,
    ) -> Vec<glam::Vec2> {
        let top_left = glam::vec2(0., 0.);
        let line_of_sight = top_left + look_at * view_distance;

        let delta = 2.0 * fov / n_rays as f32;
        let (start, end) = (
            (-(n_rays as f32) / 2.0) as i32,
            (n_rays as f32 / 2.0) as i32,
        );

        (start..=end)
            .map(|i| {
                let ray_rotation = i as f32 * delta;
                util::rotate_point_around_center(line_of_sight, top_left, ray_rotation)
            })
            .collect()
    }

    pub fn update(
        &mut self,
        source: &TransformComponent,
        rect_objects: &Vec<&ggez::graphics::Rect>,
    ) {
        let lines: Vec<(f32, f32, f32, f32)> = self
            .rays
            .iter()
            .map(|ray| {
                let angle = -constants::PI / 2. - util::get_vec_angle(self.look_at);
                let rotated_ray = util::rotate_point_around_center(*ray, glam::vec2(0., 0.), angle);
                (
                    source.position.x,
                    source.position.y,
                    source.position.x + rotated_ray.x,
                    source.position.y + rotated_ray.y,
                )
            })
            .collect();

        // Reset colors
        self.tmp_ray_colors
            .iter_mut()
            .for_each(|ray_color| *ray_color = ggez::graphics::Color::WHITE);

        // Get indices of rays which are colliding
        let colliding_rays_indices = lines.iter().enumerate().filter_map(|(i, line)| {
            rect_objects
                .iter()
                .any(|rect| util::line_rect_intersection(line.0, line.1, line.2, line.3, rect))
                .then_some(i)
        });

        // Set their colors to blue
        colliding_rays_indices.for_each(|i| self.tmp_ray_colors[i] = ggez::graphics::Color::BLUE);
    }
}

pub fn system(game_state: &mut GameState) {
    let transform_look_components = game_state
        .guards
        .iter_mut()
        .map(|guard| (&guard.transform, &mut guard.look));

    let aabb_objects = game_state
        .walls
        .iter()
        .map(|wall| &wall.aabb.rect)
        .collect::<Vec<&ggez::graphics::Rect>>();

    for (transform, look) in transform_look_components {
        look.update(transform, &aabb_objects);
    }
}
