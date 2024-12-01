use std::collections::HashMap;

fn main() {
    let input = include_str!("../inputs/1.txt");
    println!("part1: {}", problem_1_part_1(input));
    println!("part2: {}", problem_1_part_2(input));
}

fn problem_1_part_1(input: &str) -> i32 {
    // read input
    let (set1, set2) = read_columns_into_sorted_lists(input);
    sum_pairwise_difference(set1, set2)
}

fn problem_1_part_2(input: &str) -> i32 {
    let (vs, freq_map) = read_list_and_frequency_map(input);
    vs.iter().map(|v| v * freq_map.get(v).unwrap_or(&0)).sum()
}

fn read_columns_into_sorted_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut l1 = Vec::new();
    let mut l2 = Vec::new();
    input.lines().for_each(|line| {
        let nums = read_ints_from_str(line);
        l1.push(nums[0]);
        l2.push(nums[1]);
    });
    l1.sort();
    l2.sort();
    (l1, l2)
}

fn read_list_and_frequency_map(input: &str) -> (Vec<i32>, HashMap<i32, i32>) {
    let mut l = Vec::new();
    let mut freq_map = HashMap::new();
    input.lines().for_each(|line| {
        let nums = read_ints_from_str(line);
        l.push(nums[0]);
        freq_map.entry(nums[1]).and_modify(|v| *v += 1).or_insert(1);
    });
    (l, freq_map)
}

fn read_ints_from_str(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn sum_pairwise_difference(set1: Vec<i32>, set2: Vec<i32>) -> i32 {
    set1.iter()
        .zip(set2.iter())
        .map(|(x, y)| difference(*x, *y))
        .sum()
}

fn difference(num1: i32, num2: i32) -> i32 {
    if num2 > num1 {
        num2 - num1
    } else {
        num1 - num2
    }
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
