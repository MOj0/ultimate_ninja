use crate::constants;
use crate::util;
use crate::TransformComponent;
use quad_rand as qrand;

pub struct ComputeMoveComponent {
    pub rays: Vec<glam::Vec2>,
    pub ray_lines: Vec<ggez::graphics::Mesh>,
}

impl ComputeMoveComponent {
    pub fn new(
        ctx: &mut ggez::Context,
        quad_ctx: &mut ggez::miniquad::GraphicsContext,
        n_rays: usize,
        ray_length: f32,
    ) -> Self {
        let ray = glam::vec2(0., 1.) * ray_length;
        let angle_delta = 2. * constants::PI / (n_rays as f32);

        let rays = (0..n_rays)
            .map(|i| {
                let ray_rotation = i as f32 * angle_delta;
                util::rotate_point_around_center(ray, glam::vec2(0., 0.), ray_rotation)
            })
            .collect::<Vec<glam::Vec2>>();

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

        Self { rays, ray_lines }
    }

    // Retruns a move direction vector, it is not guaranteed to be normalized
    pub fn get_move_direction(
        &self,
        source: &TransformComponent,
        dist_to_wall_norm: f32,
        rect_objects: &Vec<(&ggez::graphics::Rect, isize)>,
    ) -> glam::Vec2 {
        if dist_to_wall_norm < 1. {
            return util::vec_from_angle(source.angle + (1. - dist_to_wall_norm) * constants::PI);
        }

        let ray_lines: Vec<(f32, f32, f32, f32)> = self
            .rays
            .iter()
            .map(|ray| {
                (
                    source.position.x,
                    source.position.y,
                    source.position.x + ray.x,
                    source.position.y + ray.y,
                )
            })
            .collect();

        let source_grid_idx = util::compute_grid_index(&source.position);
        let row = (constants::MAX_WORLD_X as usize / constants::GRID_CELL_SIZE) as isize;
        let source_grid_indices = vec![
            source_grid_idx - row - 1,
            source_grid_idx - row,
            source_grid_idx - row + 1,
            source_grid_idx - 1,
            source_grid_idx,
            source_grid_idx + 1,
            source_grid_idx + row - 1,
            source_grid_idx + row,
            source_grid_idx + row + 1,
        ];

        let ray_grid_indices = ray_lines
            .iter()
            .map(|(_, _, dest_x, dest_y)| util::compute_grid_index(&glam::vec2(*dest_x, *dest_y)))
            .chain(source_grid_indices.into_iter())
            .collect::<Vec<isize>>();

        let ray_collisions: Vec<bool> = ray_lines
            .iter()
            .map(|ray_line| {
                rect_objects
                    .iter()
                    .filter(|(_, object_grid_idx)| ray_grid_indices.contains(object_grid_idx))
                    .filter_map(|(rect, _)| {
                        let intersect_point = util::line_rect_intersection(
                            ray_line.0, ray_line.1, ray_line.2, ray_line.3, rect,
                        );

                        if let Some(point) = intersect_point {
                            return Some(point);
                        }
                        None
                    })
                    .min_by_key(|intersect_point| {
                        ((ray_line.0 - intersect_point.x).abs()
                            + (ray_line.1 - intersect_point.y).abs()) as u32
                    })
                    .is_some()
            })
            .collect();

        let available_rays = self
            .rays
            .iter()
            .enumerate()
            .filter_map(|(i, ray)| (!ray_collisions[i]).then(|| *ray))
            .collect::<Vec<glam::Vec2>>();

        if available_rays.len() == 0 {
            let rand_idx = qrand::gen_range(0, self.rays.len());
            return self.rays[rand_idx];
        }

        let rand_idx = qrand::gen_range(0, available_rays.len());
        return available_rays[rand_idx];
    }
}
