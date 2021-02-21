use crate::game::{consts, coords::Coords, direction::Direction, line::Line, turn::Turn};
use ggez::graphics::Rect;
use std::collections::VecDeque;

use super::segment::Segment;

/// Snake structure that consists of a list of either
/// straight or curved segments and the direction of the head.
///
pub struct Snake {
    pub body: VecDeque<Box<dyn Segment>>,
    pub dir: Direction,
}

impl Snake {
    /// Create new `Snake` of the default length
    /// on the `(x, y)` position, pointing down.
    ///
    pub fn new(x: f32, y: f32) -> Self {
        let first = Line {
            beg: Coords::new(x, y - consts::SNAKE_START_HEIGHT / 2.),
            end: Coords::new(x, y + consts::SNAKE_START_HEIGHT / 2.),
            dir: Direction::DOWN,
        };
        let boxed: Box<dyn Segment> = Box::new(first);
        let mut body = VecDeque::new();
        body.push_back(boxed);

        Self {
            body,
            dir: Direction::DOWN,
        }
    }

    /// Move `Snake` in the current direction by a given distance.
    ///
    pub fn do_move(&mut self, dist: f32) {
        self.grow(dist);
        self.shrink(dist);
    }

    fn shrink(&mut self, dist: f32) {
        let back = self.body.back_mut().unwrap();
        let shrink_left = back.shrink(dist);
        if shrink_left > 0. {
            self.body.pop_back();
            self.body.back_mut().unwrap().shrink(shrink_left);
        }
    }

    /// Extend `Snake` towards the current direction by a given distance.
    /// When `Snake` direction changes, add a new Turn at the begining.
    /// When Turn at the begining is fully extended, add a new Line at the begining.
    ///
    pub fn grow(&mut self, dist: f32) {
        let mut front = self.body.front_mut().unwrap();

        if front.direction() != self.dir {
            let pos = front.end();
            let in_dir = front.direction();
            self.body
                .push_front(Box::new(Turn::new(pos, in_dir, self.dir)));
            front = self.body.front_mut().unwrap();
        }

        let growth_left = front.grow(dist);
        if growth_left > 0. {
            let pos = front.end();
            let dir = front.direction();
            self.body.push_front(Box::new(Line::new(pos, dir)));
            self.body.front_mut().unwrap().grow(growth_left);
        }
    }

    /// Check if any of the segments collides with given `Rect`.
    ///
    pub fn collide(&self, other: &Rect) -> bool {
        self.body.iter().any(|segment| segment.collision(other))
    }

    /// Check if head is colliding with screen boundaries.
    ///
    pub fn wall_collide(&self) -> bool {
        let head = self.body.front().unwrap().bounding_box();
        head.left() < -consts::WALL_MARGIN
            || head.top() < -consts::WALL_MARGIN
            || head.bottom() > consts::SCREEN_SIZE.y + consts::WALL_MARGIN
            || head.right() > consts::SCREEN_SIZE.x + consts::WALL_MARGIN
    }

    /// Check if head is colliding with any other segment.
    ///
    pub fn self_collide(&self) -> bool {
        let head = self.body.front().unwrap();
        self.body
            .iter()
            .skip(1)
            .map(|x| x.bounding_box())
            .any(|x| head.collision(&x))
    }
}
