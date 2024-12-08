use std::collections::{HashMap, HashSet};

type Rules = HashMap<String, HashSet<String>>;

pub fn part_1(input: &str) -> Option<i32> {
    let (part_1, part_2) = input.split_once("\n\n").unwrap();
    let mut sum = 0;
    let rules = rules(part_1);
    for line in part_2.lines() {
        if let Some(middle) = get_middle_if_correct(line, &rules) {
            sum += middle;
        }
    }
    Some(sum)
}

fn rules(input: &str) -> Rules {
    let mut rules = HashMap::new();
    for line in input.lines() {
        let (key, value) = line.split_once("|").unwrap();
        let key = key.to_string();
        let set = rules.entry(key.clone()).or_insert(HashSet::new());
        set.insert(value.to_string());
    }
    rules
}

fn pages(input: &str) -> Vec<i32> {
    input
        .split(",")
        .map(|page| page.parse::<i32>().unwrap())
        .collect()
}

fn middle_page(pages: Vec<i32>) -> i32 {
    let middle = pages.len() / 2;
    pages[middle]
}

fn correct_order(pages: Vec<i32>, rules: &Rules) -> bool {
    let mut seen = HashSet::new();
    let empty = HashSet::new();
    for page in &pages {
        let cannot_be = rules.get(&page.to_string()).unwrap_or(&empty);
        if seen.intersection(cannot_be).count() > 0 {
            return false;
        }
        seen.insert(page.to_string());
    }
    true
}

fn fix_order(pages: &[i32], rules: &Rules) -> Vec<i32> {
    let empty = HashSet::new();
    let mut seen = HashSet::new();
    let mut fixed = pages.to_vec();
    for (i, page) in pages.iter().enumerate() {
        let cannot_be = rules.get(&page.to_string()).unwrap_or(&empty);
        let mut intersection = seen.intersection(cannot_be);
        if let Some(out_of_order_page) = intersection.next() {
            let oop = out_of_order_page.parse::<i32>().unwrap();
            let oop_index = pages.iter().position(|&x| x == oop).unwrap();
            fixed.swap(i, oop_index);
        }
        seen.insert(page.to_string());
    }
    fixed
}

fn get_middle_if_correct(input: &str, rules: &Rules) -> Option<i32> {
    let pages = pages(input);
    if correct_order(pages.clone(), &rules) {
        Some(middle_page(pages))
    } else {
        None
    }
}

pub fn part_2(input: &str) -> Option<i32> {
    let (part_1, part_2) = input.split_once("\n\n").unwrap();
    let mut sum = 0;
    let rules = rules(part_1);

    // rules define a topological sort edges, for each line, sort the pages
    // if the order is incorrect, find the middle page

    for line in part_2.lines() {
        if let Some(middle) = get_middle_if_incorrect(line, &rules) {
            sum += middle;
        }
    }

    Some(sum)
}

fn get_middle_if_incorrect(input: &str, rules: &Rules) -> Option<i32> {
    let pages = pages(input);
    if !correct_order(pages.clone(), &rules) {
        let fixed_pages = fix_order(&pages, &rules);
        println!("{:?} becomes {:?}", pages, fixed_pages);
        Some(middle_page(fixed_pages))
    } else {
        None
    }
}

fn fix_order(pages: &[i32], rules: &Rules) -> Vec<i32> {
    // topologically sort the pages according to the rules
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
        assert_eq!(result, Some(123));
    }
}
