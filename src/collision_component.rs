pub struct AABBCollisionComponent {
    pub rect: ggez::graphics::Rect,
    pub colliding_axis: (bool, bool),
}

impl AABBCollisionComponent {
    pub fn new(rect: ggez::graphics::Rect) -> Self {
        Self {
            rect,
            colliding_axis: (false, false),
        }
    }

    #[inline]
    pub fn check_collision(&self, other_rect: &ggez::graphics::Rect) -> bool {
        self.rect.overlaps(&other_rect)
    }

    /// Assume: self and next_move_of(entity_rect) are colliding
    /// Returns: (x_axis_colliding, y_axis_colliding)
    pub fn get_colliding_axis(&self, entity_rect: &ggez::graphics::Rect) -> (bool, bool) {
        if self.rect.top() <= entity_rect.bottom()
            && self.rect.bottom() >= entity_rect.top()
            && (entity_rect.right() <= self.rect.left() || entity_rect.left() >= self.rect.right())
        {
            return (true, false);
        }
        if self.rect.right() >= entity_rect.left()
            && self.rect.left() <= entity_rect.right()
            && (entity_rect.bottom() <= self.rect.top() || entity_rect.top() >= self.rect.bottom())
        {
            return (false, true);
        }

        self.rect
            .overlaps(entity_rect)
            .then(|| (true, true))
            .unwrap_or((false, false))
    }
}
