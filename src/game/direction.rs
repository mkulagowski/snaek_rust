use crate::game::coords::Coords;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    pub fn inverse(&self) -> Self {
        match self {
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
            Direction::LEFT => Direction::RIGHT,
            Direction::RIGHT => Direction::LEFT,
        }
    }

    pub fn as_coords(&self) -> Coords {
        let (x, y) = match self {
            Direction::UP => (0., -1.),
            Direction::DOWN => (0., 1.),
            Direction::LEFT => (-1., 0.),
            Direction::RIGHT => (1., 0.),
        };
        Coords { x, y }
    }
}
