use aoc2025::*;

fn part1(input: &str) -> i32 {
    // TODO: Implement part 1
    0
}

fn part2(input: &str) -> i32 {
    // TODO: Implement part 2
    0
}

fn main() {
    println!("Day 2 - Advent of Code 2025");
    
    // Read input lines
    let lines = read_input_lines(2).expect("Could not read input file");
    
    // Or read as single string if preferred
    // let input = read_input(2).expect("Could not read input file");
    
    let result1 = part1(&lines.join("\n"));
    let result2 = part2(&lines.join("\n"));
    
    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
}