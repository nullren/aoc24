use std::collections::{HashMap, VecDeque};

pub struct Rules {
    dependencies: HashMap<i32, Vec<i32>>,
}

impl Rules {
    pub fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
        }
    }

    pub fn add_dependency(&mut self, before: i32, after: i32) {
        self.dependencies.entry(before).or_default().push(after);
    }
}

fn topological_sort(pages: &[i32], rules: &Rules) -> Vec<i32> {
    let mut in_degree = HashMap::new();
    let mut graph = HashMap::new();

    for &page in pages {
        in_degree.insert(page, 0);
        graph.insert(page, Vec::new());
    }

    for (&before, afters) in &rules.dependencies {
        for &after in afters {
            if let Some(degree) = in_degree.get_mut(&after) {
                *degree += 1;
            }
            graph.entry(before).or_default().push(after);
        }
    }

    let mut queue = VecDeque::new();
    for (&page, &degree) in &in_degree {
        if degree == 0 {
            queue.push_back(page);
        }
    }

    let mut sorted = Vec::new();
    while let Some(page) = queue.pop_front() {
        sorted.push(page);
        if let Some(neighbors) = graph.get(&page) {
            for &neighbor in neighbors {
                if let Some(degree) = in_degree.get_mut(&neighbor) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    sorted
}

pub fn fix_order(pages: &[i32], rules: &Rules) -> Vec<i32> {
    topological_sort(pages, rules)
}
