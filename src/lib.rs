use std::ops;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vec2D(pub isize, pub isize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pos(pub usize, pub usize);

impl ops::Add<Vec2D> for Pos {
    type Output = Option<Pos>;
    fn add(self, rhs: Vec2D) -> Self::Output {
        Some(Pos(
            self.0.checked_add_signed(rhs.0)?,
            self.1.checked_add_signed(rhs.1)?,
        ))
    }
}

impl ops::Mul<isize> for Vec2D {
    type Output = Vec2D;
    fn mul(self, rhs: isize) -> Self::Output {
        Vec2D(self.0 * rhs, self.1 * rhs)
    }
}

pub const DIRECTIONS: [Vec2D; 8] = [
    Vec2D(1, 0),
    Vec2D(1, 1),
    Vec2D(0, 1),
    Vec2D(-1, 1),
    Vec2D(-1, 0),
    Vec2D(-1, -1),
    Vec2D(0, -1),
    Vec2D(1, -1),
];

pub const CROSS_DIRECTIONS: [Vec2D; 4] = [Vec2D(1, 1), Vec2D(1, -1), Vec2D(-1, 1), Vec2D(-1, -1)];
pub const UNIT_DIRECTIONS: [Vec2D; 4] = [Vec2D(1, 0), Vec2D(-1, 0), Vec2D(0, 1), Vec2D(0, -1)];
