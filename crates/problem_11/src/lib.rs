use std::collections::HashMap;

pub fn part_1(input: &str) -> Option<usize> {
    let inputs = input
        .split_ascii_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let freq = freq_map(&inputs);
    let mut solver = Solver::default();
    Some(tally(&solver.solve(&freq)))
}

// let MEMO: HashMap<u64, Vec<u64>> = HashMap::new();

#[derive(Default)]
struct Solver {
    memo: HashMap<u64, Vec<u64>>,
}

impl Solver {
    pub fn solve_25(&mut self, input: u64) -> Vec<u64> {
        if let Some(v) = self.memo.get(&input) {
            return v.clone();
        }
        let mut inputs = vec![input];
        for _ in 0..25 {
            inputs = apply_rules(&inputs);
        }
        self.memo.insert(input, inputs.clone());
        inputs
    }

    pub fn solve(&mut self, freq: &HashMap<u64, usize>) -> HashMap<u64, usize> {
        freq.iter().fold(HashMap::new(), |mut acc, (k, v)| {
            let new_inputs = self.solve_25(*k);
            for n in new_inputs {
                *acc.entry(n).or_insert(0) += v;
            }
            acc
        })
    }
}

fn freq_map(inputs: &[u64]) -> HashMap<u64, usize> {
    inputs.iter().fold(HashMap::new(), |mut acc, &n| {
        *acc.entry(n).or_insert(0) += 1;
        acc
    })
}

fn tally(inputs: &HashMap<u64, usize>) -> usize {
    inputs.values().sum()
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

pub fn part_2(input: &str) -> Option<usize> {
    let inputs = input
        .split_ascii_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let freq = freq_map(&inputs);
    let mut solver = Solver::default();
    let freq = solver.solve(&freq); // 25
    let freq = solver.solve(&freq); // 50
    let freq = solver.solve(&freq); // 75
    Some(tally(&freq))
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
