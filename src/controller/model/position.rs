use std::ops::{Add, Sub};

use strum_macros::EnumIter;

use super::board::{HEIGHT, WIDTH};

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl<T, U> From<(T, U)> for Position
where
    T: Into<usize>,
    U: Into<usize>,
{
    fn from(value: (T, U)) -> Self {
        Self {
            x: value.0.into(),
            y: value.1.into(),
        }
    }
}

impl From<Position> for (usize, usize) {
    fn from(value: Position) -> Self {
        (value.x, value.y)
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        self.apply(usize::add, rhs)
            .apply(usize::rem_euclid, (WIDTH, HEIGHT).into())
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        (
            (self.x as isize - rhs.x as isize).rem_euclid(WIDTH as isize) as usize,
            (self.y as isize - rhs.y as isize).rem_euclid(HEIGHT as isize) as usize,
        )
            .into()
    }
}

impl Position {
    fn apply(self, func: impl Fn(usize, usize) -> usize, other: Position) -> Position {
        Position {
            x: func(self.x, other.x),
            y: func(self.y, other.y),
        }
    }

    pub fn apply_one(self, func: impl Fn(usize, usize) -> usize, other: usize) -> Position {
        Position {
            x: func(self.x, other),
            y: func(self.y, other),
        }
    }

    pub fn move_in_direction(self, dir: Direction) -> Position {
        let delta = match dir {
            Direction::Up | Direction::Down => (0_usize, 1_usize).into(),
            Direction::Left | Direction::Right => (1_usize, 0_usize).into(),
        };
        match dir {
            Direction::Left | Direction::Up => self - delta,
            Direction::Right | Direction::Down => self + delta,
        }
    }

    pub fn dist(self, other: Position) -> Position {
        (
            (self.x as isize - other.x as isize)
                .rem_euclid(WIDTH as isize)
                .min((other.x as isize - self.x as isize).rem_euclid(WIDTH as isize))
                as usize,
            (self.y as isize - other.y as isize)
                .rem_euclid(HEIGHT as isize)
                .min((other.y as isize - self.y as isize).rem_euclid(HEIGHT as isize))
                as usize,
        )
            .into()
    }

    pub fn signed_dist(self, other: Position) -> (isize, isize) {
        let s = |a, b, m| {
            -(a as isize - b as isize - m as isize / 2).rem_euclid(m as isize) + m as isize / 2
        };
        (s(self.x, other.x, WIDTH), s(self.y, other.y, HEIGHT))
    }

    pub fn mag(self) -> usize {
        self.x + self.y
    }

    pub fn reduce<T>(self, f: impl Fn(usize, usize) -> T) -> T {
        f(self.x, self.y)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
