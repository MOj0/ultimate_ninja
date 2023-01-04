use crate::constants;
use crate::transform_component::TransformComponent;
use crate::util;
use crate::Game;

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
        let fov_mesh_composition = Self::mesh_composition_from_rays(ctx, quad_ctx, &rays);

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
        let line_of_sight = look_at * view_distance;

        let delta = 2.0 * fov / n_rays as f32;
        let (start, end) = (
            (-(n_rays as f32) / 2.0) as i32,
            (n_rays as f32 / 2.0) as i32,
        );

        (start..=end)
            .map(|i| {
                let ray_rotation = i as f32 * delta;
                util::rotate_point_around_center(line_of_sight, glam::vec2(0., 0.), ray_rotation)
            })
            .collect()
    }

    fn mesh_composition_from_rays(
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::miniquad::GraphicsContext,
        rays: &Vec<glam::Vec2>,
    ) -> Vec<ggez::graphics::Mesh> {
        return rays
            .windows(2)
            .map(|fov_rays| {
                let fov_points = vec![glam::vec2(0., 0.), fov_rays[0], fov_rays[1]];

                ggez::graphics::Mesh::new_polygon(
                    ctx,
                    quad_ctx,
                    ggez::graphics::DrawMode::fill(),
                    &fov_points,
                    ggez::graphics::Color::from_rgba(255, 255, 255, 125),
                )
                .unwrap()
            })
            .collect::<Vec<ggez::graphics::Mesh>>();
    }

    pub fn update(
        &mut self,
        source: &TransformComponent,
        rect_objects: &Vec<(&ggez::graphics::Rect, isize)>,
    ) -> Vec<(usize, f32)> {
        let ray_lines: Vec<(f32, f32, f32, f32)> = self
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
        let wall_idx_and_ray_scales: Vec<(Option<usize>, f32)> = ray_lines
            .iter()
            .enumerate()
            .map(|(ray_idx, ray_line)| {
                rect_objects
                    .iter()
                    .enumerate()
                    .filter_map(|(wall_idx, (rect, _))| {
                        let intersect_point = util::line_rect_intersection(
                            ray_line.0, ray_line.1, ray_line.2, ray_line.3, rect,
                        );

                        if let Some(point) = intersect_point {
                            return Some((wall_idx, point));
                        }
                        None
                    })
                    .min_by_key(|(_, intersect_point)| {
                        ((ray_line.0 - intersect_point.x).abs()
                            + (ray_line.1 - intersect_point.y).abs()) as u32
                    })
                    .map_or_else(
                        || (None, 1.0),
                        |(wall_idx, min_intersection_point)| {
                            (
                                Some(wall_idx),
                                (glam::vec2(ray_line.0, ray_line.1) - min_intersection_point)
                                    .length()
                                    / self.rays[ray_idx].length(),
                            )
                        },
                    )
            })
            .collect();

        for (ray_idx, (_, ray_scale)) in wall_idx_and_ray_scales.iter().enumerate() {
            self.ray_scales[ray_idx] = *ray_scale;
        }

        wall_idx_and_ray_scales
            .iter()
            .filter(|(wall_idx, _)| wall_idx.is_some())
            .map(|(wall_idx, ray_scale)| (wall_idx.unwrap(), *ray_scale))
            .collect::<Vec<(usize, f32)>>()
    }
}

pub fn system(game_state: &mut Game) {
    let transform_look_basic = game_state.guards_basic.iter_mut().map(|guard| {
        (
            &guard.guard.transform,
            &mut guard.guard.look_components[guard.guard.look_idx],
        )
    });

    let transform_look_scout = game_state.guards_scout.iter_mut().map(|guard| {
        (
            &guard.guard.transform,
            &mut guard.guard.look_components[guard.guard.look_idx],
        )
    });

    let transform_look_components = transform_look_basic.chain(transform_look_scout);

    let aabb_objects = game_state
        .walls
        .iter()
        .map(|wall| (&wall.aabb.rect, wall.transform.grid_index))
        .collect::<Vec<(&ggez::graphics::Rect, isize)>>();

    let mut brightness_vec = vec![constants::GLOBAL_BRIGHTNESS; game_state.walls.len()];
    for (transform, look) in transform_look_components {
        let wall_idx_and_ray_scales = look.update(transform, &aabb_objects);

        for (wall_idx, ray_scale) in wall_idx_and_ray_scales.iter() {
            brightness_vec[*wall_idx] = (1.5 - *ray_scale).min(1.);
        }
    }

    game_state
        .target
        .look
        .update(&game_state.target.transform, &aabb_objects);

    for (wall_idx, brightness) in brightness_vec.iter().enumerate() {
        game_state.walls[wall_idx].set_brightness(*brightness);
    }
}
