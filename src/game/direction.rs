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
    /// Check if `self` and `other` are colinear,
    /// i.e. if they are on the same axis (X/Y)
    ///
    pub fn is_colinear(&self, other: Self) -> bool {
        match self {
            Self::UP | Self::DOWN => matches!(other, Self::UP | Self::DOWN),
            Self::LEFT | Self::RIGHT => matches!(other, Self::LEFT | Self::RIGHT),
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
