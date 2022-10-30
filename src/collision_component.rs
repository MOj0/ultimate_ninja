use crate::transform_component::TransformComponent;

pub struct CollisionComponent {
    pub size: f32,
}

// TODO: Finish this collision component (but it is easier to just use Transform component??)
impl CollisionComponent {
    pub fn new(size: f32) -> Self {
        Self { size }
    }

    pub fn is_player_detected(
        &self,
        transform: &TransformComponent,
        player_transform: &TransformComponent,
    ) -> bool {
        let vec_to_player = player_transform.position - transform.position;
        let len_to_player_sq = vec_to_player.length_squared();

        return len_to_player_sq <= (player_transform.size + transform.size).powi(2);
    }
}
