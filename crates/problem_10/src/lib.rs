pub fn part_1(input: &str) -> Option<i32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut scores = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '0' {
                scores += trails(&grid, '0', row, col)
                    .iter()
                    .collect::<std::collections::HashSet<_>>()
                    .len();
            }
        }
    }

    Some(scores as i32)
}

fn trails(grid: &[Vec<char>], current: char, row: usize, col: usize) -> Vec<(usize, usize)> {
    if current == '9' {
        return vec![(row, col)];
    }

    // find next step
    let next = (current as u8 + 1) as char;

    let mut ends: Vec<(usize, usize)> = vec![];
    if row > 0 && grid[row - 1][col] == next {
        ends.extend_from_slice(&trails(grid, next, row - 1, col));
    }

    if row < grid.len() - 1 && grid[row + 1][col] == next {
        ends.extend_from_slice(&trails(grid, next, row + 1, col));
    }

    if col > 0 && grid[row][col - 1] == next {
        ends.extend_from_slice(&trails(grid, next, row, col - 1));
    }

    if col < grid[0].len() - 1 && grid[row][col + 1] == next {
        ends.extend_from_slice(&trails(grid, next, row, col + 1));
    }

    ends
}

pub fn part_2(input: &str) -> Option<i32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut scores = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '0' {
                scores += rating(&grid, '0', row, col);
            }
        }
    }
    Some(scores as i32)
}

fn rating(grid: &[Vec<char>], current: char, row: usize, col: usize) -> usize {
    if current == '9' {
        return 1;
    }

    // find next step
    let next = (current as u8 + 1) as char;

    let mut count = 0;
    if row > 0 && grid[row - 1][col] == next {
        count += rating(grid, next, row - 1, col);
    }

    if row < grid.len() - 1 && grid[row + 1][col] == next {
        count += rating(grid, next, row + 1, col);
    }

    if col > 0 && grid[row][col - 1] == next {
        count += rating(grid, next, row, col - 1);
    }

    if col < grid[0].len() - 1 && grid[row][col + 1] == next {
        count += rating(grid, next, row, col + 1);
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT);
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT);
        assert_eq!(result, Some(81));
    }
}
