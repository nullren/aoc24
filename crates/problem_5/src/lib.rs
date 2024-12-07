use std::collections::{HashMap, HashSet};

pub fn part_1(_input: &str) -> Option<i32> {
    None
}

fn rules(input: &str) -> HashMap<String, HashSet<String>> {
    let mut rules = HashMap::new();
    for line in input.lines() {
        let (key, value) = line.split_once("|").unwrap();
        let key = key.to_string();
        let set = rules.entry(key.clone()).or_insert(HashSet::new());
        set.insert(value.to_string());
    }
    rules
}

pub fn part_2(_input: &str) -> Option<i32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT);
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_split_parts() {
        let (part_1, part_2) = INPUT.split_once("\n\n").unwrap();
        assert_eq!(part_1.lines().count(), 21);
        assert_eq!(part_2.lines().count(), 6);
    }

    #[test]
    fn test_rules_parsing() {
        let (part_1, _) = INPUT.split_once("\n\n").unwrap();
        let rules = rules(part_1);
        assert_eq!(rules.len(), 6);
        assert_eq!(rules.get("47").unwrap().len(), 4);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT);
        assert_eq!(result, None);
    }
}
