pub mod guard;
pub mod player;
pub mod target;

use crate::move_component::MoveComponent;
use crate::transform_component::TransformComponent;

pub fn move_entity(transform: &mut TransformComponent, move_component: &MoveComponent) {
    transform.update(move_component.direction * move_component.speed);
}
