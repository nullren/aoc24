pub fn part_1(input: &str) -> Option<i32> {
    input
        .split_ascii_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .map(|word| {
            if word == 0 {
                return vec![1];
            }
            if word.len() % 2 == 0 {
                // split word in half, remove leading 0s
            }
        })
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
