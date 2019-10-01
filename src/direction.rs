#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn all_except(self) -> [Direction; 3] {
        match self {
            Direction::Up => [Direction::Down, Direction::Left, Direction::Right],
            Direction::Down => [Direction::Up, Direction::Left, Direction::Right],
            Direction::Left => [Direction::Up, Direction::Down, Direction::Right],
            Direction::Right => [Direction::Up, Direction::Down, Direction::Left],
        }
    }
}
