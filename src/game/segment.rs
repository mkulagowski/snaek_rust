use ggez::{graphics::Rect, Context};

use super::{consts, coords::Coords, direction::Direction};

/// Trait for growth functionality of the snake segments
///
pub trait Growable {
    fn grow(&mut self, dist: f32) -> f32;
    fn shrink(&mut self, dist: f32) -> f32;
    fn end(&self) -> Coords;
    fn direction(&self) -> Direction;
}

/// Trait for rendering and collision functionality of the snake segments
///
pub trait Renderable {
    fn draw(&self, ctx: &mut Context);
    fn bounding_box(&self) -> Rect;

    fn collision(&self, other: &Rect) -> bool {
        let rect = self.bounding_box();

        if rect.left() >= other.right()
            || rect.right() <= other.left()
            || rect.top() >= other.bottom()
            || rect.bottom() <= other.top()
        {
            return false;
        }

        let ll = f32::max(rect.left(), other.left());
        let rr = f32::min(rect.right(), other.right());
        let tt = f32::max(rect.top(), other.top());
        let bb = f32::min(rect.bottom(), other.bottom());

        (rr - ll) * (bb - tt) >= consts::COLLISION_PIXELS_MARGIN
    }
}

pub trait Segment: Growable + Renderable {}

impl<T: Growable + Renderable> Segment for T {}
