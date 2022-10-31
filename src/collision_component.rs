pub struct AABBCollisionComponent {
    pub rect: ggez::graphics::Rect,
    pub colliding_components: (bool, bool),
}

impl AABBCollisionComponent {
    pub fn new(rect: ggez::graphics::Rect) -> Self {
        Self {
            rect,
            colliding_components: (false, false),
        }
    }

    #[inline]
    pub fn check_collision(&self, other_rect: &ggez::graphics::Rect) -> bool {
        self.rect.overlaps(&other_rect)
    }

    /// Assume: self and other_rect are colliding
    pub fn get_colliding_components(&self, entity_rect: &ggez::graphics::Rect) -> (bool, bool) {
        if entity_rect.right() <= self.rect.left() || entity_rect.left() >= self.rect.right() {
            return (true, false);
        }
        if entity_rect.bottom() <= self.rect.top() || entity_rect.top() >= self.rect.bottom() {
            return (false, true);
        }
        (false, false)
    }
}
