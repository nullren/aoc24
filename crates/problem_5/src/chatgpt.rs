use std::collections::{HashMap, HashSet, VecDeque};

fn parse_rule(line: &str) -> (i32, i32) {
    let parts: Vec<&str> = line.split('|').collect();
    let left = parts[0].trim().parse::<i32>().unwrap();
    let right = parts[1].trim().parse::<i32>().unwrap();
    (left, right)
}

fn parse_update(line: &str) -> Vec<i32> {
    line.split(',')
        .map(|part| part.trim().parse::<i32>().unwrap())
        .collect()
}

fn is_correct_order(update: &[i32], constraints: &[(i32, i32)]) -> bool {
    let pos: HashMap<i32, usize> = update.iter().enumerate().map(|(i, &p)| (p, i)).collect();
    for &(x, y) in constraints {
        if let (Some(&px), Some(&py)) = (pos.get(&x), pos.get(&y)) {
            if px > py {
                return false;
            }
        }
    }
    true
}

fn topological_sort(pages: &HashSet<i32>, constraints: &[(i32, i32)]) -> Vec<i32> {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut indegree: HashMap<i32, usize> = HashMap::new();

    for &p in pages {
        indegree.insert(p, 0);
        graph.insert(p, Vec::new());
    }

    for &(x, y) in constraints {
        if pages.contains(&x) && pages.contains(&y) {
            graph.get_mut(&x).unwrap().push(y);
            *indegree.get_mut(&y).unwrap() += 1;
        }
    }

    let mut queue: VecDeque<i32> = VecDeque::new();
    for (&page, &deg) in &indegree {
        if deg == 0 {
            queue.push_back(page);
        }
    }

    let mut result = Vec::new();
    while let Some(node) = queue.pop_front() {
        result.push(node);
        if let Some(neighbors) = graph.get(&node) {
            for &nxt in neighbors {
                let d = indegree[&nxt] - 1;
                *indegree.get_mut(&nxt).unwrap() = d;
                if d == 0 {
                    queue.push_back(nxt);
                }
            }
        }
    }

    result
}

fn middle_page(pages: &[i32]) -> i32 {
    pages[pages.len() / 2]
}

pub fn solve(input: &str) -> i32 {
    // Split input into rules and updates by finding a blank line
    let sections: Vec<&str> = input.split("\n\n").collect();
    let rules_section = sections[0].trim();
    let updates_section = sections[1].trim();

    let all_rules: Vec<(i32, i32)> = rules_section.lines().map(parse_rule).collect();
    let updates: Vec<Vec<i32>> = updates_section.lines().map(parse_update).collect();

    let mut incorrect_updates = Vec::new();
    let mut correct_updates = Vec::new();

    for upd in &updates {
        let relevant_constraints: Vec<(i32, i32)> = all_rules
            .iter()
            .copied()
            .filter(|(x, y)| upd.contains(x) && upd.contains(y))
            .collect();

        if is_correct_order(upd, &relevant_constraints) {
            correct_updates.push(upd.clone());
        } else {
            incorrect_updates.push((upd.clone(), relevant_constraints));
        }
    }

    // Now topologically sort the incorrect updates
    let mut fixed_updates = Vec::new();
    for (upd, constraints) in incorrect_updates {
        let pages: HashSet<i32> = upd.iter().copied().collect();
        let correct_order = topological_sort(&pages, &constraints);
        fixed_updates.push(correct_order);
    }

    // Sum the middle pages of the fixed (incorrect) updates
    fixed_updates.iter().map(|u| middle_page(u)).sum()
}
