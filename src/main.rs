use std::collections::HashMap;

fn main() {
    let input = include_str!("../inputs/1.txt");
    println!("part1: {}", problem_1_part_1(input));
    println!("part2: {}", problem_1_part_2(input));
}

fn problem_1_part_1(input: &str) -> i32 {
    let mut columns = vec![
        ColumnType::ListInts(Vec::new()),
        ColumnType::ListInts(Vec::new()),
    ];
    read_into_columns(input, &mut columns);

    columns.iter_mut().for_each(|c| c.sort());
    columns[0]
        .as_slice()
        .iter()
        .zip(columns[1].as_slice().iter())
        .map(|(x, y)| (x - y).abs())
        .sum()
}

fn problem_1_part_2(input: &str) -> i32 {
    let mut columns = vec![
        ColumnType::ListInts(Vec::new()),
        ColumnType::FrequencyMapInts(HashMap::new()),
    ];
    read_into_columns(input, &mut columns);

    let vs = columns[0].as_slice();
    let freq_map = columns[1].as_freq_map();
    vs.iter().map(|v| v * freq_map.get(v).unwrap_or(&0)).sum()
}

#[derive(Debug)]
enum ColumnType {
    ListInts(Vec<i32>),
    FrequencyMapInts(HashMap<i32, i32>),
}

impl ColumnType {
    fn insert(&mut self, num: i32) {
        match self {
            ColumnType::ListInts(l) => l.push(num),
            ColumnType::FrequencyMapInts(freq_map) => {
                freq_map.entry(num).and_modify(|v| *v += 1).or_insert(1);
            }
        }
    }

    fn as_slice(&self) -> &[i32] {
        match self {
            ColumnType::ListInts(l) => l.as_slice(),
            _ => panic!("Not a list"),
        }
    }

    fn sort(&mut self) {
        match self {
            ColumnType::ListInts(l) => l.sort(),
            _ => panic!("Not a list"),
        }
    }

    fn as_freq_map(&self) -> &HashMap<i32, i32> {
        match self {
            ColumnType::FrequencyMapInts(freq_map) => freq_map,
            _ => panic!("Not a frequency map"),
        }
    }
}

fn read_into_columns(input: &str, columns: &mut [ColumnType]) {
    input.lines().for_each(|line| {
        let nums = read_ints_from_str(line);
        nums.iter().enumerate().for_each(|(i, &num)| {
            columns[i].insert(num);
        });
    });
}

fn read_ints_from_str(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use crate::{problem_1_part_1, problem_1_part_2};

    #[test]
    fn test_problem_1_part_1() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3
";
        assert_eq!(problem_1_part_1(input), 11);
    }

    #[test]
    fn test_problem_1_part_2() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3
";
        assert_eq!(problem_1_part_2(input), 31);
    }
}
