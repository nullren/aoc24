pub fn part_1(input: &str) -> i32 {
    input
        .lines()
        .filter(|line: &&str| {
            let line = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            level_safe(&line)
        })
        .count() as i32
}

pub fn part_2(input: &str) -> i32 {
    input
        .lines()
        .filter(|line: &&str| level_safe_dampened(line))
        .count() as i32
}

fn level_safe(line: &[i32]) -> bool {
    let mut prev: Option<i32> = None;
    let mut increasing = None;
    for num in line {
        let num = *num;
        if let Some(prev) = prev {
            let diff = (num - prev).abs();
            if !(1..=3).contains(&diff) {
                return false;
            }
            if let Some(increasing) = increasing {
                if increasing {
                    if num < prev {
                        return false;
                    }
                } else if num > prev {
                    return false;
                }
            } else {
                increasing = Some(num > prev);
            }
        }
        prev = Some(num);
    }
    true
}

fn level_safe_dampened(line: &str) -> bool {
    let nums = line
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    if level_safe(&nums) {
        return true;
    }

    for i in 0..nums.len() {
        let mut nums = nums.clone();
        nums.remove(i);
        if level_safe(&nums) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(part_1(input), 2);
    }

    #[test]
    fn test_part_2() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(part_2(input), 4);
    }
}
