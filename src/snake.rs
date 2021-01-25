use crate::{
    consts,
    coords::Coords,
    direction::Direction,
    line::Line,
    turn::{Turn, TurnType},
};
use ggez::graphics::Rect;
use std::collections::LinkedList;

#[derive(Debug)]
pub struct LineSnake {
    pub body: LinkedList<Line>,
    pub dir: Direction,
}

impl LineSnake {
    pub fn new(x: f32, y: f32) -> Self {
        let mut body = LinkedList::new();
        body.push_back(Line {
            beg: Coords::new(x, y - consts::SNAKE_START_HEIGHT / 2.),
            end: Coords::new(x, y + consts::SNAKE_START_HEIGHT / 2.),
            dir: Direction::DOWN,
            turn: None,
        });

        Self {
            body,
            dir: Direction::DOWN,
        }
    }

    pub fn do_move(&mut self, dist: f32) {
        self.grow(dist);
        let back = self.body.back_mut().unwrap();
        let shrink_left = back.shrinkage_left(dist);
        if shrink_left == 0.0 {
            back.shrink(dist);
        } else {
            self.body.pop_back();
            self.body.back_mut().unwrap().shrink(shrink_left);
        }
    }

    pub fn grow(&mut self, dist: f32) {
        let front = self.body.front().unwrap();
        let front_dir = front.dir;
        let dir = self.dir;
        if dir != front_dir {
            let mut new_coords = front.end.clone();
            new_coords += front.dir.as_coords() * consts::SNAKE_HALF_WIDTH;
            new_coords += dir.inverse().as_coords() * consts::SNAKE_HALF_WIDTH;

            self.body.push_front(Line {
                beg: new_coords,
                end: new_coords,
                dir: dir,
                turn: Some(Turn::new(TurnType::from_dirs(&front_dir, &dir), new_coords)),
            });
        }
        self.body.front_mut().unwrap().grow(dist);
    }

    pub fn collide(&self, other: &Rect) -> bool {
        self.body.front().unwrap().get_rekt().overlaps(other)
    }

    pub fn wall_collide(&self) -> bool {
        let head = self.body.front().unwrap().get_rekt();
        head.left() < -consts::WALL_MARGIN
            || head.top() < -consts::WALL_MARGIN
            || head.bottom() > consts::SCREEN_SIZE.y + consts::WALL_MARGIN
            || head.right() > consts::SCREEN_SIZE.x + consts::WALL_MARGIN
    }

    pub fn self_collide(&self) -> bool {
        let head = self.body.front().unwrap();
        self.body
            .iter()
            .skip(1)
            .map(|x| x.get_rekt())
            .any(|x| head.overlaps(&x))
    }
}
