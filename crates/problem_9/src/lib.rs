pub fn part_1(input: &str) -> Option<i32> {
    let nums = parse_input(input);
    let result = checksum(&nums);
    Some(result as i32)
}

fn parse_input(input: &str) -> Vec<u8> {
    input
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u8))
        .collect()
}

fn checksum(nums: &[u8]) -> u64 {
    let mut sum = 0;
    for (i, &num) in nums.iter().enumerate() {
        sum += i as u64 * num as u64;
    }
    sum
}

pub fn part_2(_input: &str) -> Option<i32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0099811188827773336446555566";

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT);
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT);
        assert_eq!(result, None);
    }
}
