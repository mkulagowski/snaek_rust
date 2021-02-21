use ggez::{
    graphics::{self, Rect},
    Context,
};

#[cfg(feature = "debug")]
use ggez::graphics::{Color, Mesh};

use crate::game::{coords::Coords, direction::Direction};

use super::{
    consts,
    segment::{Growable, Renderable},
    Renderer,
};

/// Curved segment of a snake, 0-90 degrees of a ring.
///
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Turn {
    pub percentage: f32,
    pub is_growing: bool,
    pub pos: Coords,
    pub in_dir: Direction,
    pub out_dir: Direction,
}

impl Turn {
    /// Create a new `Turn` that starts on the given `pos` with `in_dir`
    /// and turns towards `out_dir`.
    ///
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
        if !self.is_growing || self.percentage >= 1. {
            return dist;
        }

        let left = f32::clamp(
            dist - (1. - self.percentage) * consts::SNAKE_WIDTH,
            0.,
            dist,
        );
        self.percentage = f32::clamp(self.percentage + dist / consts::SNAKE_WIDTH, 0., 1.);
        self.is_growing = self.percentage < 1.;

        left
    }

    fn shrink(&mut self, dist: f32) -> f32 {
        if self.percentage <= 0. {
            return dist;
        }

        let left = f32::clamp(dist - self.percentage * consts::SNAKE_WIDTH, 0., dist);
        self.percentage = f32::clamp(self.percentage - dist / consts::SNAKE_WIDTH, 0., 1.);

        left
    }

    fn end(&self) -> Coords {
        self.pos
            + self.in_dir.as_coords() * consts::SNAKE_HALF_WIDTH
            + self.out_dir.as_coords() * consts::SNAKE_HALF_WIDTH
    }

    fn direction(&self) -> Direction {
        self.out_dir
    }
}

impl Renderable for Turn {
    fn bounding_box(&self) -> Rect {
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
        if let Ok(mesh) = Renderer::create_qt_ring(
            ctx,
            pos,
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

/// Enum that describes a Turn as one of the quaters of the circle
///
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TurnType {
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl TurnType {
    /// Create `TurnType` based on the Turn directions
    ///
    pub fn from_dirs(in_dir: &Direction, out_dir: &Direction) -> Self {
        if (in_dir == &Direction::LEFT && out_dir == &Direction::UP)
            || (in_dir == &Direction::DOWN && out_dir == &Direction::RIGHT)
        {
            TurnType::DownRight
        } else if (in_dir == &Direction::DOWN && out_dir == &Direction::LEFT)
            || (in_dir == &Direction::RIGHT && out_dir == &Direction::UP)
        {
            TurnType::DownLeft
        } else if (in_dir == &Direction::RIGHT && out_dir == &Direction::DOWN)
            || (in_dir == &Direction::UP && out_dir == &Direction::LEFT)
        {
            TurnType::UpLeft
        } else if (in_dir == &Direction::UP && out_dir == &Direction::RIGHT)
            || (in_dir == &Direction::LEFT && out_dir == &Direction::DOWN)
        {
            TurnType::UpRight
        } else {
            unreachable!()
        }
    }

    /// Convert `TurnType` into pair of start->end angles.
    ///
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

/// Get translation for each quater of the circle
///
pub fn _get_arc_translation(quarter: TurnType) -> Coords {
    match quarter {
        TurnType::DownRight => Coords::new(0.5, -0.5),
        TurnType::DownLeft => Coords::new(-0.5, -0.5),
        TurnType::UpLeft => Coords::new(-0.5, 0.5),
        TurnType::UpRight => Coords::new(0.5, 0.5),
    }
}
