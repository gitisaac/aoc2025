use aoc2025::*;
use log::{debug, info};

fn concatenate_digits(left: u32, right: u32) -> i32 {
    let concatenated = format!("{}{}", left, right);
    concatenated.parse().unwrap()
}

fn find_max_with_index(digits: &Vec<u32>) -> Option<(usize, u32)> {
    if digits.is_empty() {
        return None;
    }
    
    let mut max_value = digits[0];
    let mut max_index = 0;
    
    for (i, &value) in digits.iter().enumerate() {
        if value > max_value {
            max_value = value;
            max_index = i;
        }
    }
    
    Some((max_index, max_value))
}

fn calc_maximum_joltage(bank: &String) -> i32 {
    let digits: Vec<u32> = bank.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    // remove last digit when finding max_value of the left value
    let search_slice = if digits.len() > 1 { &digits[..digits.len()-1] } else { &digits };
    if let Some((max_index, max_value_left)) = find_max_with_index(&search_slice.to_vec()) {        
        // Slice from the element after max_index to the end to find the max left value
        let remaining_slice = &digits[max_index + 1..];
        debug!("found max_value_left {} at {}", max_value_left, max_index);
        if let Some((ii, max_value_right)) = find_max_with_index(&remaining_slice.to_vec()) {
            debug!("found max_value_right {} at {}", max_value_right, ii);
            concatenate_digits(max_value_left, max_value_right)
        } else {
            panic!("No elements in remaining slice");
        }
    } else {
        panic!("Empty vector!")
    }
}

fn part1(banks: &Vec<String>) -> i32 {
    let mut cum_sum = 0;
    for bank in banks {
        let max_joltage = calc_maximum_joltage(bank);
        debug!("max joltage {} for {}", max_joltage, bank);
        cum_sum += max_joltage;
    }
    cum_sum
}

fn part2(banks: &Vec<String>) -> i32 {
    0
}

fn main() {
    env_logger::init();
    
    debug!("Day 3 - Advent of Code 2025");
    
    // read input as a single string
    let input_list = read_input_lines(3).expect("Could not read input");
    //let input_list = read_sample_lines(3).expect("Could not read input");
    
    let result1 = part1(&input_list);
    let result2 = part2(&input_list);

    info!("Part 1: {}", result1);
    info!("Part 2: {}", result2);
}
