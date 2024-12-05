pub fn part_1(input: &str) -> Option<i32> {
    let grid = Grid::new(input);

    let mut xmas_count = 0;
    for (y, row) in grid.enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == 'X' {
                xmas_count += grid.count_xmases_at(x, y);
            }
        }
    }
    Some(xmas_count)
}

struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        Self { grid }
    }

    fn enumerate(&self) -> impl Iterator<Item = (usize, &Vec<char>)> {
        self.grid.iter().enumerate()
    }

    fn xmas_up(&self, x: usize, y: usize) -> bool {
        if y < 3 {
            return false;
        }
        self.grid[y][x] == 'X'
            && self.grid[y - 1][x] == 'M'
            && self.grid[y - 2][x] == 'A'
            && self.grid[y - 3][x] == 'S'
    }

    fn xmas_down(&self, x: usize, y: usize) -> bool {
        if y > self.grid.len() - 4 {
            return false;
        }
        self.grid[y][x] == 'X'
            && self.grid[y + 1][x] == 'M'
            && self.grid[y + 2][x] == 'A'
            && self.grid[y + 3][x] == 'S'
    }

    fn xmas_left(&self, x: usize, y: usize) -> bool {
        if x < 3 {
            return false;
        }
        self.grid[y][x] == 'X'
            && self.grid[y][x - 1] == 'M'
            && self.grid[y][x - 2] == 'A'
            && self.grid[y][x - 3] == 'S'
    }

    fn xmas_right(&self, x: usize, y: usize) -> bool {
        if x > self.grid[0].len() - 4 {
            return false;
        }
        self.grid[y][x] == 'X'
            && self.grid[y][x + 1] == 'M'
            && self.grid[y][x + 2] == 'A'
            && self.grid[y][x + 3] == 'S'
    }

    fn xmas_up_left(&self, x: usize, y: usize) -> bool {
        if y < 3 || x < 3 {
            return false;
        }
        self.grid[y][x] == 'X'
            && self.grid[y - 1][x - 1] == 'M'
            && self.grid[y - 2][x - 2] == 'A'
            && self.grid[y - 3][x - 3] == 'S'
    }

    fn xmas_up_right(&self, x: usize, y: usize) -> bool {
        if y < 3 || x > self.grid[0].len() - 4 {
            return false;
        }
        self.grid[y][x] == 'X'
            && self.grid[y - 1][x + 1] == 'M'
            && self.grid[y - 2][x + 2] == 'A'
            && self.grid[y - 3][x + 3] == 'S'
    }

    fn xmas_down_left(&self, x: usize, y: usize) -> bool {
        if y > self.grid.len() - 4 || x < 3 {
            return false;
        }
        self.grid[y][x] == 'X'
            && self.grid[y + 1][x - 1] == 'M'
            && self.grid[y + 2][x - 2] == 'A'
            && self.grid[y + 3][x - 3] == 'S'
    }

    fn xmas_down_right(&self, x: usize, y: usize) -> bool {
        if y > self.grid.len() - 4 || x > self.grid[0].len() - 4 {
            return false;
        }
        self.grid[y][x] == 'X'
            && self.grid[y + 1][x + 1] == 'M'
            && self.grid[y + 2][x + 2] == 'A'
            && self.grid[y + 3][x + 3] == 'S'
    }

    fn count_xmases_at(&self, x: usize, y: usize) -> i32 {
        [
            self.xmas_up(x, y),
            self.xmas_down(x, y),
            self.xmas_left(x, y),
            self.xmas_right(x, y),
            self.xmas_up_left(x, y),
            self.xmas_up_right(x, y),
            self.xmas_down_left(x, y),
            self.xmas_down_right(x, y),
        ]
        .iter()
        .filter(|&&x| x)
        .count() as i32
    }

    fn mas_at(&self, x: usize, y: usize) -> bool {
        if y < 1 || y > self.grid.len() - 2 || x < 1 || x > self.grid[0].len() - 2 {
            return false;
        }
        self.grid[y][x] == 'A'
            && ((self.grid[y - 1][x - 1] == 'M' && self.grid[y + 1][x + 1] == 'S')
                || (self.grid[y - 1][x - 1] == 'S' && self.grid[y + 1][x + 1] == 'M'))
            && ((self.grid[y - 1][x + 1] == 'M' && self.grid[y + 1][x - 1] == 'S')
                || (self.grid[y - 1][x + 1] == 'S' && self.grid[y + 1][x - 1] == 'M'))
    }
}

pub fn part_2(input: &str) -> Option<i32> {
    let grid = Grid::new(input);

    let mut mas_count = 0;
    for (y, row) in grid.enumerate() {
        for (x, _col) in row.iter().enumerate() {
            if grid.mas_at(x, y) {
                mas_count += 1;
            }
        }
    }
    Some(mas_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT);
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT);
        assert_eq!(result, Some(9));
    }
}
