use std::{fmt::Display, mem, ops};

pub struct Grid<T> {
    pub inner: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn new(inner: Vec<Vec<T>>) -> Self {
        Self { inner }
    }
    pub fn get(&self, pos: Pos) -> Option<&T> {
        self.inner.get(pos.1).and_then(|row| row.get(pos.0))
    }
    pub fn set(&mut self, pos: Pos, value: T) -> Option<T> {
        self.inner
            .get_mut(pos.1)
            .and_then(|row| row.get_mut(pos.0).map(|v| mem::replace(v, value)))
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.inner {
            for v in row {
                write!(f, "{v}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct Vec2D(pub isize, pub isize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct Pos(pub usize, pub usize);

impl Pos {
    pub fn dist(&self, other: Pos) -> Vec2D {
        Vec2D(
            other.0 as isize - self.0 as isize,
            other.1 as isize - self.1 as isize,
        )
    }
}

impl ops::Add<Vec2D> for Pos {
    type Output = Option<Pos>;
    fn add(self, rhs: Vec2D) -> Self::Output {
        Some(Pos(
            self.0.checked_add_signed(rhs.0)?,
            self.1.checked_add_signed(rhs.1)?,
        ))
    }
}

impl ops::Sub<Vec2D> for Pos {
    type Output = Option<Pos>;
    fn sub(self, rhs: Vec2D) -> Self::Output {
        Some(Pos(
            self.0.checked_add_signed(rhs.0 * -1)?,
            self.1.checked_add_signed(rhs.1 * -1)?,
        ))
    }
}

impl Pos {
    pub fn wrapping_add(self, rhs: Vec2D, bounds: (usize, usize)) -> Pos {
        Pos(
            (self.0 + bounds.0.checked_add_signed(rhs.0).unwrap()) % bounds.0,
            (self.1 + bounds.1.checked_add_signed(rhs.1).unwrap()) % bounds.1,
        )
    }
}

impl ops::Mul<isize> for Vec2D {
    type Output = Vec2D;
    fn mul(self, rhs: isize) -> Self::Output {
        Vec2D(self.0 * rhs, self.1 * rhs)
    }
}

pub const COMPASS_DIRECTIONS: [Vec2D; 8] = [
    Vec2D(1, 0),
    Vec2D(1, 1),
    Vec2D(0, 1),
    Vec2D(-1, 1),
    Vec2D(-1, 0),
    Vec2D(-1, -1),
    Vec2D(0, -1),
    Vec2D(1, -1),
];

pub const ORDINAL_DIRECTIONS: [Vec2D; 4] = [Vec2D(1, 1), Vec2D(1, -1), Vec2D(-1, 1), Vec2D(-1, -1)];
pub const CARDINAL_DIRECTIONS: [Vec2D; 4] = [Vec2D(1, 0), Vec2D(-1, 0), Vec2D(0, 1), Vec2D(0, -1)];

pub const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction::Up),
            '>' => Ok(Direction::Right),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            c => Err(format!("invalid direction {c}")),
        }
    }
}

impl Direction {
    pub fn unit_direction(&self) -> Vec2D {
        match self {
            Direction::Up => Vec2D(0, -1),
            Direction::Down => Vec2D(0, 1),
            Direction::Left => Vec2D(-1, 0),
            Direction::Right => Vec2D(1, 0),
        }
    }

    pub fn turned_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    pub fn turned_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    pub fn reversed(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
