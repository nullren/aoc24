use std::collections::{HashMap, HashSet};

mod coord;

use coord::Position;

pub fn part_1(input: &str) -> Option<i32> {
    let mut cols = 0;
    let mut rows = 0;
    let mut char_locs = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if row > rows {
                rows = row;
            }
            if col > cols {
                cols = col;
            }
            if c != '.' {
                char_locs
                    .entry(c)
                    .or_insert_with(Vec::new)
                    .push(Position::new(row as i64, col as i64));
            }
        }
    }
    // println!("map dimensions: {} {}", rows, cols);
    // println!("character locations: {:?}", char_locs);

    let mut antinodes = HashSet::new();
    for locs in char_locs.values() {
        for i in 0..locs.len() {
            for j in 0..locs.len() {
                if i == j {
                    continue;
                }
                let antinode = locs[j].antinode(locs[i]);
                if antinode.row >= 0
                    && antinode.row <= rows as i64
                    && antinode.col >= 0
                    && antinode.col <= cols as i64
                {
                    antinodes.insert(antinode);
                }
                let antinode = locs[i].antinode(locs[j]);
                if antinode.row >= 0
                    && antinode.row <= rows as i64
                    && antinode.col >= 0
                    && antinode.col <= cols as i64
                {
                    antinodes.insert(antinode);
                }
            }
        }
    }
    // println!("antinodes: {:?}", antinodes);

    Some(antinodes.len() as i32)
}

pub fn part_2(_input: &str) -> Option<i32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT);
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT);
        assert_eq!(result, None);
    }
}
