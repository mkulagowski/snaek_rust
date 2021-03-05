use crate::game::coords::Coords;

/// Enumeration for 4 main directions
///
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    /// Return inverted direction.
    ///
    pub fn inverse(&self) -> Self {
        match self {
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
            Direction::LEFT => Direction::RIGHT,
            Direction::RIGHT => Direction::LEFT,
        }
    }

    /// Return direction as a 2d versor
    ///
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
