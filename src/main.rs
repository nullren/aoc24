fn main() {
    let input = include_str!("../inputs/1.txt");
    println!("{}", problem_1(input));
}

fn problem_1(input: &str) -> i32 {
    // read input
    let (set1, set2) = read_columns_into_sorted_lists(input);
    sum_pairwise_difference(set1, set2)
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
    use crate::problem_1;

    #[test]
    fn test_problem_1() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3
";
        assert_eq!(problem_1(input), 11);
    }
}
