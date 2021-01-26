use ggez::{
    graphics::{self, Mesh, Rect},
    Context,
};

use crate::game::{consts, coords::Coords, direction::Direction, turn::Turn};

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Line {
    pub beg: Coords,
    pub end: Coords,
    pub dir: Direction,
    pub turn: Option<Turn>,
    //pub is_turn: bool,
}

impl Line {
    pub fn grow(&mut self, dist: f32) {
        match self.dir {
            Direction::UP => self.end.y -= dist,
            Direction::DOWN => self.end.y += dist,
            Direction::LEFT => self.end.x -= dist,
            Direction::RIGHT => self.end.x += dist,
        };

        if let Some(ref mut turn) = self.turn {
            if turn.is_growing && turn.percentage < 1. {
                turn.percentage = clamp(turn.percentage + dist / consts::SNAKE_WIDTH, 0., 1.);
                turn.is_growing = turn.percentage < 1.;
            }
        }
    }

    pub fn shrink(&mut self, dist: f32) {
        match self.dir {
            Direction::UP => self.beg.y -= dist,
            Direction::DOWN => self.beg.y += dist,
            Direction::LEFT => self.beg.x -= dist,
            Direction::RIGHT => self.beg.x += dist,
        }

        if let Some(ref mut turn) = self.turn {
            if turn.percentage > 0. {
                turn.percentage = clamp(turn.percentage - dist / consts::SNAKE_WIDTH, 0., 1.);
            }
        }
    }

    pub fn get_rekt(&self) -> Rect {
        let (x, y, w, h) = match self.dir {
            Direction::UP => (
                self.end.x - consts::SNAKE_HALF_WIDTH,
                self.end.y,
                consts::SNAKE_WIDTH,
                (self.end.y - self.beg.y).abs(),
            ),
            Direction::DOWN => (
                self.end.x - consts::SNAKE_HALF_WIDTH,
                self.beg.y,
                consts::SNAKE_WIDTH,
                (self.end.y - self.beg.y).abs(),
            ),
            Direction::LEFT => (
                self.end.x,
                self.end.y - consts::SNAKE_HALF_WIDTH,
                (self.end.x - self.beg.x).abs(),
                consts::SNAKE_WIDTH,
            ),
            Direction::RIGHT => (
                self.beg.x,
                self.end.y - consts::SNAKE_HALF_WIDTH,
                (self.end.x - self.beg.x).abs(),
                consts::SNAKE_WIDTH,
            ),
        };

        Rect::new(x, y, w, h)
    }

    pub fn size(&self) -> f32 {
        match self.dir {
            Direction::UP | Direction::DOWN => (self.end.y - self.beg.y).abs(),
            Direction::LEFT | Direction::RIGHT => (self.end.x - self.beg.x).abs(),
        }
    }

    pub fn shrinkage_left(&self, dist: f32) -> f32 {
        let shrinked_wall = self.size();
        if shrinked_wall > dist {
            0.0
        } else {
            dist - shrinked_wall
        }
    }

    pub fn create_mesh(&self, ctx: &mut Context, color: graphics::Color) -> Mesh {
        Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), self.get_rekt(), color).unwrap()
    }

    pub fn overlaps(&self, other: &Rect) -> bool {
        let rect = self.get_rekt();

        if rect.left() < other.right()
            && rect.right() > other.left()
            && rect.top() < other.bottom()
            && rect.bottom() > other.top()
        {
            let ll = maxf(rect.left(), other.left());
            let rr = minf(rect.right(), other.right());
            let tt = maxf(rect.top(), other.top());
            let bb = minf(rect.bottom(), other.bottom());

            return (rr - ll) * (bb - tt) >= consts::COLLISION_PIXELS_MARGIN;
        }
        false
    }
}

fn maxf(x: f32, y: f32) -> f32 {
    if x > y {
        x
    } else {
        y
    }
}

fn minf(x: f32, y: f32) -> f32 {
    if x > y {
        y
    } else {
        x
    }
}

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x > max {
        max
    } else if x < min {
        min
    } else {
        x
    }
}
