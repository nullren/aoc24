use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: i64,
    pub col: i64,
}

impl Position {
    pub fn new(row: i64, col: i64) -> Self {
        Self { row, col }
    }

    pub fn scale(&self, factor: i64) -> Self {
        Self {
            row: self.row.saturating_mul(factor),
            col: self.col.saturating_mul(factor),
        }
    }

    pub fn antinode(&self, other: Self) -> Self {
        self.scale(2) - other
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            row: self.row.saturating_add(other.row),
            col: self.col.saturating_add(other.col),
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            row: self.row.saturating_sub(other.row),
            col: self.col.saturating_sub(other.col),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_add() {
        let pos1 = Position::new(1, 2);
        let pos2 = Position::new(3, 4);
        let result = pos1 + pos2;
        assert_eq!(result, Position::new(4, 6));
    }

    #[test]
    fn test_sub() {
        let pos1 = Position::new(1, 2);
        let pos2 = Position::new(3, 4);
        let result = pos2 - pos1;
        assert_eq!(result, Position::new(2, 2));
    }

    #[test]
    fn test_antinode() {
        let pos1 = Position::new(1, 1);
        let pos2 = Position::new(2, 2);
        let result = pos2.antinode(pos1);
        assert_eq!(result, Position::new(3, 3));
    }
}
