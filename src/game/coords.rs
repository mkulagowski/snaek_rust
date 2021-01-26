use rand::distributions::{Distribution, Uniform};
use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Coords {
    pub x: f32,
    pub y: f32,
}

impl Coords {
    pub fn new(x: f32, y: f32) -> Self {
        Coords { x, y }
    }

    pub fn random(min: f32, max: f32) -> Self {
        let mut rng = rand::thread_rng();
        let xrand = Uniform::from(min..max);
        let yrand = Uniform::from(min..max);
        Self {
            x: xrand.sample(&mut rng),
            y: yrand.sample(&mut rng),
        }
    }
}

impl AddAssign for Coords {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Add for Coords {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Sub for Coords {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul for Coords {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Mul<f32> for Coords {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        Self::Output {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Mul<Coords> for f32 {
    type Output = Coords;

    fn mul(self, other: Coords) -> Self::Output {
        Self::Output {
            x: self * other.x,
            y: self * other.y,
        }
    }
}

impl Into<ggez::mint::Point2<f32>> for Coords {
    fn into(self) -> ggez::mint::Point2<f32> {
        ggez::mint::Point2 {
            x: self.x,
            y: self.y,
        }
    }
}
