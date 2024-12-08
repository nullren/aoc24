pub fn part_1(_input: &str) -> Option<i32> {
    None
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
        assert_eq!(result, None);
    }
}
