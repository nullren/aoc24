mod problem_1;
mod problem_2;

fn main() {
    let input = include_str!("../inputs/2.txt");
    println!("part1: {}", problem_2::part_1(input));
    println!("part2: {}", problem_2::part_2(input));
}
