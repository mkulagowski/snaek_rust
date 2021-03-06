use crate::game::coords::Coords;

/// Enumeration for 4 main directions
///
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Check if `self` and `other` are colinear,
    /// i.e. if they are on the same axis (X/Y)
    ///
    pub fn is_colinear(&self, other: Self) -> bool {
        match self {
            Self::Up | Self::Down => matches!(other, Self::Up | Self::Down),
            Self::Left | Self::Right => matches!(other, Self::Left | Self::Right),
        }
    }

    /// Return direction as a 2d versor
    ///
    pub fn as_coords(&self) -> Coords {
        let (x, y) = match self {
            Direction::Up => (0., -1.),
            Direction::Down => (0., 1.),
            Direction::Left => (-1., 0.),
            Direction::Right => (1., 0.),
        };
        Coords { x, y }
    }
}
