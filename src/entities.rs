pub mod guard;
pub mod player;
pub mod target;
pub mod wall;

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
