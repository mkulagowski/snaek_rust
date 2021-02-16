use ggez::{graphics::Rect, Context};

use super::{consts, coords::Coords, direction::Direction, maths};

pub trait Growable {
    fn grow(&mut self, dist: f32) -> f32;
    fn shrink(&mut self, dist: f32) -> f32;
    fn get_end(&self) -> Coords;
    fn get_dir(&self) -> Direction;
}
pub trait Renderable {
    fn draw(&self, ctx: &mut Context);
    fn get_bbox(&self) -> Rect;

    fn collision(&self, other: &Rect) -> bool {
        let rect = self.get_bbox();
        if rect.left() < other.right()
            && rect.right() > other.left()
            && rect.top() < other.bottom()
            && rect.bottom() > other.top()
        {
            let ll = maths::maxf(rect.left(), other.left());
            let rr = maths::minf(rect.right(), other.right());
            let tt = maths::maxf(rect.top(), other.top());
            let bb = maths::minf(rect.bottom(), other.bottom());

            return (rr - ll) * (bb - tt) >= consts::COLLISION_PIXELS_MARGIN;
        }
        false
    }
}

pub trait Segment: Growable + Renderable {}
