#[derive(Clone, Copy, Default)]
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
        Self{
            x: value.0.into(),
            y: value.1.into(),
        }
    }
}

impl From<Position> for (usize, usize){
    fn from(value: Position) -> Self {
        (value.x, value.y)
    }
}

impl Position{
    pub fn apply(self, func: impl Fn(usize, usize) -> usize, other: Position) -> Position{
        Position { x: func(self.x, other.x), y: func(self.y, other.y) }
    }

    pub fn apply_one(self, func: impl Fn(usize, usize) -> usize, other: usize) -> Position{
        Position { x: func(self.x, other), y: func(self.y, other) }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction{
    Up,
    Down,
    Left,
    Right,
}
