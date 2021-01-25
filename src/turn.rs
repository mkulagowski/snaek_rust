use crate::{coords::Coords, direction::Direction};

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Turn {
    pub turn_type: TurnType,
    pub percentage: f32,
    pub is_growing: bool,
    pub pos: Coords,
}

impl Turn {
    pub fn new(turn_type: TurnType, pos: Coords) -> Self {
        Self {
            turn_type,
            percentage: 0.,
            is_growing: true,
            pos: pos,
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
