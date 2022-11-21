use crate::constants;
use crate::transform_component::TransformComponent;
use crate::util;
use crate::GameState;

pub struct LookComponent {
    pub look_at: glam::Vec2,
    pub fov: f32,
    pub view_distance: f32,
    pub fov_mesh_composition: Vec<ggez::graphics::Mesh>,
    pub rays: Vec<glam::Vec2>,
    pub ray_lines: Vec<ggez::graphics::Mesh>,
    pub ray_scales: Vec<f32>,
}

impl LookComponent {
    pub fn new(look_at: glam::Vec2, fov: f32, view_distance: f32, n_rays: u32) -> Self {
        let rays = Self::make_rays(look_at, fov, view_distance, n_rays);
        let fov_mesh_composition = vec![];
        let ray_lines = vec![];

        let ray_scales = rays.iter().map(|_| 1.0).collect::<Vec<f32>>();

        Self {
            look_at,
            fov,
            view_distance,
            fov_mesh_composition,
            rays,
            ray_lines,
            ray_scales,
        }
    }

    pub fn new_with_mesh(
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::miniquad::GraphicsContext,
        look_at: glam::Vec2,
        fov: f32,
        view_distance: f32,
        n_rays: u32,
    ) -> Self {
        let rays = Self::make_rays(look_at, fov, view_distance, n_rays);
        let fov_mesh_composition =
            Self::make_fov_mesh_composition(ctx, quad_ctx, look_at, view_distance, &rays);

        // Build a line mesh for each ray
        let ray_lines = rays
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

        let ray_scales = rays.iter().map(|_| 1.0).collect::<Vec<f32>>();

        Self {
            look_at,
            fov,
            view_distance,
            fov_mesh_composition,
            rays,
            ray_lines,
            ray_scales,
        }
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

    fn make_fov_mesh_composition(
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::miniquad::GraphicsContext,
        look_at: glam::Vec2,
        view_distance: f32,
        rays: &Vec<glam::Vec2>,
    ) -> Vec<ggez::graphics::Mesh> {
        let top_left = glam::vec2(0., 0.);

        return rays
            .windows(2)
            .map(|fov_rays| {
                // Swapping these values makes a cool effect :D
                let fov_corner1 = fov_rays[1];
                let fov_corner2 = fov_rays[0];
                let intersection = (fov_corner1 - fov_corner2) / 2. + fov_corner2;

                let arc_points = util::get_arc_points(
                    intersection,
                    glam::vec2(
                        ((fov_corner1 - fov_corner2) / 2.).length(),
                        view_distance - intersection.length(),
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

                ggez::graphics::Mesh::new_polygon(
                    ctx,
                    quad_ctx,
                    ggez::graphics::DrawMode::fill(),
                    &fov_points,
                    ggez::graphics::Color::from_rgba(127, 0, 0, 127),
                )
                .unwrap()
            })
            .collect::<Vec<ggez::graphics::Mesh>>();
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

        // Get and set the scale [0.0, 1.0] for each ray
        self.ray_scales = lines
            .iter()
            .map(|line| {
                rect_objects
                    .iter()
                    .filter_map(|rect| {
                        util::line_rect_intersection(line.0, line.1, line.2, line.3, rect)
                    })
                    .min_by_key(|intersect_point| {
                        ((line.0 - intersect_point.x).abs() + (line.1 - intersect_point.y).abs())
                            as u32
                    })
                    .map_or_else(
                        || 1.0,
                        |min_intersection_point| {
                            let scale = (glam::vec2(line.0, line.1) - min_intersection_point)
                                .length()
                                / (glam::vec2(line.0, line.1) - glam::vec2(line.2, line.3))
                                    .length();
                            scale.min(1.0)
                        },
                    )
            })
            .collect::<Vec<f32>>();
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

    game_state
        .target
        .look
        .update(&game_state.target.transform, &aabb_objects);
}
