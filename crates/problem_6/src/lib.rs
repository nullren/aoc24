use std::collections::HashSet;

pub fn part_1(input: &str) -> Option<i32> {
    let world = World::new(input);
    let guard = world.find_guard();
    Some(run(&world, guard) as i32)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
    fn peek(&self, grid: &[Vec<char>]) -> Option<(char, Position)> {
        let row = self.position.row;
        let col = self.position.col;

        match self.direction {
            Direction::Up => {
                if row == 0 {
                    None
                } else {
                    Some((grid[row - 1][col], Position::new(row - 1, col)))
                }
            }
            Direction::Down => {
                if row == grid.len() - 1 {
                    None
                } else {
                    Some((grid[row + 1][col], Position::new(row + 1, col)))
                }
            }
            Direction::Left => {
                if col == 0 {
                    None
                } else {
                    Some((grid[row][col - 1], Position::new(row, col - 1)))
                }
            }
            Direction::Right => {
                if col == grid[0].len() - 1 {
                    None
                } else {
                    Some((grid[row][col + 1], Position::new(row, col + 1)))
                }
            }
        }
    }

    /// Returns the next guard position, otherwise None if the guard is looking at the edge of the world.
    fn next(&self, grid: &[Vec<char>]) -> Option<Self> {
        let peek = self.peek(grid);
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

#[derive(Debug, Clone)]
struct World {
    grid: Vec<Vec<char>>,
}

impl World {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        Self { grid }
    }

    fn find_guard(&self) -> Guard {
        for (row, line) in self.grid.iter().enumerate() {
            for (col, &cell) in line.iter().enumerate() {
                if let Ok(direction) = Direction::try_from(cell) {
                    let position = Position::new(row, col);
                    return Guard::new(position, direction);
                }
            }
        }
        panic!("guard not found");
    }

    fn with_distraction(&self, distraction: Position) -> Self {
        let mut world = self.clone();
        world.grid[distraction.row][distraction.col] = '#';
        world
    }
}

fn run(world: &World, guard: Guard) -> usize {
    let mut path = HashSet::new();
    path.insert(guard.position);
    let mut guard = guard;
    loop {
        let next = guard.next(&world.grid);
        match next {
            Some(next_guard) => {
                guard = next_guard;
                path.insert(guard.position);
            }
            None => break,
        }
    }
    path.len()
}

fn run_with_simulation(world: &World, mut guard: Guard) -> usize {
    let mut visited = HashSet::new();
    visited.insert(guard);

    let mut successful_distractions = 0;

    loop {
        match guard.peek(&world.grid) {
            Some(('#', _)) => {}
            None => {}
            Some((_, position)) => {
                let distracted = world.with_distraction(position);
                let mut visited = visited.clone();
                if loops(&distracted, guard, &mut visited) {
                    successful_distractions += 1;
                }
            }
        }

        guard = match guard.next(&world.grid) {
            Some(guard) => guard,
            None => break,
        };
    }

    successful_distractions
}

fn loops(world: &World, guard: Guard, visited: &mut HashSet<Guard>) -> bool {
    let next = guard.next(&world.grid);
    match next {
        Some(next) => {
            if visited.contains(&next) {
                return true;
            }
            visited.insert(next);
            loops(world, next, visited)
        }
        None => false,
    }
}

pub fn part_2(input: &str) -> Option<i32> {
    let world = World::new(input);
    let guard = world.find_guard();
    Some(run_with_simulation(&world, guard) as i32)
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
