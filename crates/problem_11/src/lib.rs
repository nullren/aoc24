pub fn part_1(input: &str) -> Option<usize> {
    let mut inputs = input
        .split_ascii_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..25 {
        inputs = apply_rules(&inputs);
    }

    Some(inputs.len())
}

fn evaluate_rules(input: u64) -> Vec<u64> {
    if input == 0 {
        return vec![1];
    }
    let input_str = input.to_string();
    if input_str.len() % 2 == 0 {
        // split word in half
        let half = input_str.len() / 2;
        let (first, second) = input_str.split_at(half);
        let first = first.parse::<u64>().unwrap();
        let second = second.parse::<u64>().unwrap();
        return vec![first, second];
    }
    vec![input * 2024]
}

fn apply_rules(inputs: &[u64]) -> Vec<u64> {
    inputs
        .iter()
        .flat_map(|input| evaluate_rules(*input))
        .collect()
}

pub fn part_2(_input: &str) -> Option<i32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "125 17";

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT);
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT);
        assert_eq!(result, None);
    }
}
