use std::collections::HashSet;

pub fn part_1(input: &str) -> Option<i32> {
    let world = World::new(input);
    let guard = world.find_guard();
    Some(run(&world, &guard).len() as i32)
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

    fn with_distraction(&self, distraction: Position) -> Option<Self> {
        let mut world = self.clone();
        if world.grid[distraction.row][distraction.col] == '.' {
            world.grid[distraction.row][distraction.col] = '#';
            Some(world)
        } else {
            None
        }
    }
}

fn run(world: &World, guard: &Guard) -> HashSet<Position> {
    let mut path = HashSet::new();
    path.insert(guard.position);
    let mut guard = *guard;
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
    path
}

fn loops(world: &World, guard: &Guard) -> bool {
    let mut path = HashSet::new();
    let mut guard = *guard;
    path.insert(guard);
    loop {
        let next = guard.next(&world.grid);
        match next {
            Some(next_guard) => {
                guard = next_guard;
                if path.contains(&guard) {
                    return true;
                }
                path.insert(guard);
            }
            None => break,
        }
    }
    false
}

pub fn part_2(input: &str) -> Option<i32> {
    let world = World::new(input);
    let guard = world.find_guard();
    let guard_places = run(&world, &guard);

    let mut guard_loops = 0;

    // if we place a distraction at any of the guard places, does it loop?
    for place in guard_places {
        if let Some(world) = world.with_distraction(place) {
            if loops(&world, &guard) {
                guard_loops += 1;
            }
        }
    }
    Some(guard_loops)
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
