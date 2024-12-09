use std::collections::HashSet;

pub fn part_1(input: &str) -> Option<i32> {
    let mut world = World::new(input);
    Some(world.run() as i32)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Guard {
    position: Position,
    direction: Direction,
}

impl Guard {
    fn new(position: Position, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }

    /// Returns the position of the space the guard is looking at,
    /// otherwise None if the guard is looking at the edge of the world.
    fn peek(&self, world: &World) -> Option<(char, Position)> {
        let row = self.position.row;
        let col = self.position.col;

        match self.direction {
            Direction::Up => {
                if row == 0 {
                    None
                } else {
                    Some((world.grid[row - 1][col], Position::new(row - 1, col)))
                }
            }
            Direction::Down => {
                if row == world.grid.len() - 1 {
                    None
                } else {
                    Some((world.grid[row + 1][col], Position::new(row + 1, col)))
                }
            }
            Direction::Left => {
                if col == 0 {
                    None
                } else {
                    Some((world.grid[row][col - 1], Position::new(row, col - 1)))
                }
            }
            Direction::Right => {
                if col == world.grid[0].len() - 1 {
                    None
                } else {
                    Some((world.grid[row][col + 1], Position::new(row, col + 1)))
                }
            }
        }
    }

    /// Returns the next guard position, otherwise None if the guard is looking at the edge of the world.
    fn next(&self, world: &World) -> Option<Self> {
        let peek = self.peek(world);
        match peek {
            Some(('#', _)) => {
                let direction = self.direction.turn_right();
                Some(Self::new(self.position, direction))
            }
            Some((_, position)) => Some(Self::new(position, self.direction)),
            None => None,
        }
    }
}

struct World {
    grid: Vec<Vec<char>>,
    guard: Guard,
}

impl World {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        // find guard
        for (row, line) in grid.iter().enumerate() {
            for (col, &cell) in line.iter().enumerate() {
                if let Ok(direction) = Direction::try_from(cell) {
                    let position = Position::new(row, col);
                    let guard = Guard::new(position, direction);
                    return Self { grid, guard };
                }
            }
        }
        panic!("guard not found");
    }

    fn run(&mut self) -> usize {
        let mut path = vec![self.guard.position];
        loop {
            let next = self.guard.next(self);
            match next {
                Some(guard) => {
                    self.guard = guard;
                    path.push(guard.position);
                }
                None => break,
            }
        }
        path.into_iter().collect::<HashSet<Position>>().len()
    }
}

pub fn part_2(_input: &str) -> Option<i32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT);
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT);
        assert_eq!(result, Some(6));
    }
}
