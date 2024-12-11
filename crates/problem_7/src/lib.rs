mod sequence;

pub fn part_1(input: &str) -> Option<i64> {
    let mut total = 0;
    for line in input.lines() {
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
        let depth = parts[0].strip_suffix(":").unwrap().parse::<i64>().unwrap();
        let nums = parts[1..]
            .iter()
            .map(|&s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        if has_valid_expression(&nums, depth) {
            total += depth;
        }
    }
    Some(total)
}

#[derive(Clone, Copy)]
enum Ops {
    Add,
    Mul,
}

fn has_valid_expression(nums: &[i64], total: i64) -> bool {
    let elems = [Ops::Add, Ops::Mul];
    let n = nums.len();
    sequence::SequenceIterator::new(&elems, n - 1).any(|ops| {
        let mut result = nums[0];
        for (i, &op) in ops.iter().enumerate() {
            match op {
                Ops::Add => result += nums[i + 1],
                Ops::Mul => result *= nums[i + 1],
            }
        }
        result == total
    })
}

pub fn part_2(_input: &str) -> Option<i32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT);
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT);
        assert_eq!(result, None);
    }
}
