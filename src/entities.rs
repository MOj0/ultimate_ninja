pub mod guard;
pub mod player;
pub mod target;
pub mod wall;

use crate::collision_component::AABBCollisionComponent;
use crate::move_component::MoveComponent;
use crate::transform_component::TransformComponent;

pub fn move_entity(
    transform: &mut TransformComponent,
    move_component: &MoveComponent,
    colliding_components: (bool, bool),
) {
    let mut move_dir = move_component.direction * move_component.speed;
    if colliding_components.0 {
        move_dir.x = 0.;
    }
    if colliding_components.1 {
        move_dir.y = 0.;
    }

    transform.update(move_dir);
}

#[inline]
pub fn get_rect_of_next_move(
    transform: &TransformComponent,
    move_component: &MoveComponent,
    aabb: &AABBCollisionComponent,
) -> ggez::graphics::Rect {
    let new_pos = transform.position + move_component.direction * move_component.speed;
    ggez::graphics::Rect::new(new_pos.x, new_pos.y, aabb.rect.w, aabb.rect.h)
}
