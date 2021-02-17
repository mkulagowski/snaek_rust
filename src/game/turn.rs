use ggez::{
    graphics::{self, Rect},
    Context,
};

#[cfg(feature = "debug")]
use ggez::graphics::{Color, Mesh};

use crate::game::{coords::Coords, direction::Direction};

use super::{
    consts, maths,
    segment::{Growable, Renderable, Segment},
    Renderer,
};

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Turn {
    pub percentage: f32,
    pub is_growing: bool,
    pub pos: Coords,
    pub in_dir: Direction,
    pub out_dir: Direction,
}

impl Segment for Turn {}

impl Turn {
    pub fn new(pos: Coords, in_dir: Direction, out_dir: Direction) -> Self {
        Self {
            percentage: 0.,
            is_growing: true,
            pos,
            in_dir,
            out_dir,
        }
    }
}

impl Growable for Turn {
    fn grow(&mut self, dist: f32) -> f32 {
        if self.is_growing && self.percentage < 1. {
            let left = maths::clamp(
                dist - (1. - self.percentage) * consts::SNAKE_WIDTH,
                0.,
                dist,
            );
            self.percentage = maths::clamp(self.percentage + dist / consts::SNAKE_WIDTH, 0., 1.);
            self.is_growing = self.percentage < 1.;
            return left;
        }
        dist
    }

    fn shrink(&mut self, dist: f32) -> f32 {
        if self.percentage > 0. {
            let left = maths::clamp(dist - self.percentage * consts::SNAKE_WIDTH, 0., dist);
            self.percentage = maths::clamp(self.percentage - dist / consts::SNAKE_WIDTH, 0., 1.);
            return left;
        }

        dist
    }

    fn get_end(&self) -> Coords {
        self.pos
            + self.in_dir.as_coords() * consts::SNAKE_HALF_WIDTH
            + self.out_dir.as_coords() * consts::SNAKE_HALF_WIDTH
    }

    fn get_dir(&self) -> Direction {
        self.out_dir
    }
}

impl Renderable for Turn {
    fn get_bbox(&self) -> Rect {
        let (x, y) = match self.in_dir {
            Direction::UP => (
                self.pos.x - consts::SNAKE_HALF_WIDTH,
                self.pos.y - (consts::SNAKE_WIDTH),
            ),
            Direction::DOWN => (self.pos.x - consts::SNAKE_HALF_WIDTH, self.pos.y),
            Direction::LEFT => (
                self.pos.x - (consts::SNAKE_WIDTH),
                self.pos.y - consts::SNAKE_HALF_WIDTH,
            ),
            Direction::RIGHT => (self.pos.x, self.pos.y - consts::SNAKE_HALF_WIDTH),
        };

        Rect::new(x, y, consts::SNAKE_WIDTH, consts::SNAKE_WIDTH)
    }

    fn draw(&self, ctx: &mut Context) {
        let turn_type = TurnType::from_dirs(&self.in_dir, &self.out_dir);
        let (margin, is_reversed) = match turn_type {
            TurnType::DownRight => (Coords { x: 1., y: -1. }, self.out_dir == Direction::UP),
            TurnType::DownLeft => (Coords { x: -1., y: -1. }, self.out_dir == Direction::LEFT),
            TurnType::UpLeft => (Coords { x: -1., y: 1. }, self.out_dir == Direction::DOWN),
            TurnType::UpRight => (Coords { x: 1., y: 1. }, self.out_dir == Direction::RIGHT),
        };

        let pos = self.pos
            + self.in_dir.as_coords() * consts::SNAKE_HALF_WIDTH
            + margin * consts::HALF_TURN_MARGIN;
        if let Some(mesh) = Renderer::create_qt_ring(
            ctx,
            &pos,
            consts::SNAKE_WIDTH + consts::TURN_MARGIN,
            consts::TURN_MARGIN,
            turn_type,
            self.percentage,
            is_reversed ^ self.is_growing,
            true,
        ) {
            graphics::draw(ctx, &mesh, graphics::DrawParam::default())
                .expect("Error while drawing Turn");
        }

        #[cfg(feature = "debug")]
        {
            let mesh = Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::stroke(1.),
                self.get_bbox(),
                Color::from_rgb(255, 0, 0),
            )
            .unwrap();
            graphics::draw(ctx, &mesh, graphics::DrawParam::default())
                .expect("Error while drawing Turn border");
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TurnType {
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl TurnType {
    pub fn from_dirs(d1: &Direction, d2: &Direction) -> Self {
        if (d1 == &Direction::LEFT && d2 == &Direction::UP)
            || (d1 == &Direction::DOWN && d2 == &Direction::RIGHT)
        {
            TurnType::DownRight
        } else if (d1 == &Direction::DOWN && d2 == &Direction::LEFT)
            || (d1 == &Direction::RIGHT && d2 == &Direction::UP)
        {
            TurnType::DownLeft
        } else if (d1 == &Direction::RIGHT && d2 == &Direction::DOWN)
            || (d1 == &Direction::UP && d2 == &Direction::LEFT)
        {
            TurnType::UpLeft
        } else if (d1 == &Direction::UP && d2 == &Direction::RIGHT)
            || (d1 == &Direction::LEFT && d2 == &Direction::DOWN)
        {
            TurnType::UpRight
        } else {
            unreachable!()
        }
    }

    pub fn get_arc_bounds(self) -> (f32, f32) {
        let from = match self {
            TurnType::DownLeft => 0.,
            TurnType::DownRight => 90.,
            TurnType::UpRight => 180.,
            TurnType::UpLeft => 270.,
        };

        (from, from + 90.)
    }
}

pub fn _get_arc_translation(quarter: TurnType) -> Coords {
    match quarter {
        TurnType::DownRight => Coords::new(0.5, -0.5),
        TurnType::DownLeft => Coords::new(-0.5, -0.5),
        TurnType::UpLeft => Coords::new(-0.5, 0.5),
        TurnType::UpRight => Coords::new(0.5, 0.5),
    }
}
