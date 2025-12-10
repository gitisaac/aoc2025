use aoc2025::*;
use log::{debug, info};

fn concatenate_digits_math(digits: &[u32]) -> i64 {
    if digits.is_empty() {
        return 0;
    }

    let mut result = 0i64;
    for &digit in digits {
        result = result * 10 + digit as i64;
    }
    result
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

fn calc_max_joltage_with_x_batteries(bank: &String, batteries: usize) -> i64 {
    let digits: Vec<u32> = bank.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let mut max_digits: Vec<u32> = Vec::new();
    let mut remaining_slice = &digits[..];
    for i in (0..batteries).rev() {
        let search_slice = if remaining_slice.len() > i {&remaining_slice[..remaining_slice.len()-i]} else {&remaining_slice};
        debug!("search_slice: {:?} (len={})", search_slice, search_slice.len());

        if let Some((max_index, max_value)) = find_max_with_index(&search_slice.to_vec()) {
            debug!("Found max_value {} at index {}", max_value, max_index);
            remaining_slice = &remaining_slice[max_index + 1..];
            debug!("remaining_slice after update: {:?}", remaining_slice);
            max_digits.push(max_value);
            debug!("max_digits so far: {:?}", max_digits);
        } else {
            panic!("No elements in remaining slice");
        }
    }
    concatenate_digits_math(&max_digits)
}

fn part1(banks: &Vec<String>) -> i64 {
    let mut cum_sum = 0;
    for bank in banks {
        let max_joltage = calc_max_joltage_with_x_batteries(bank, 2);
        debug!("max joltage {} for {}", max_joltage, bank);
        cum_sum += max_joltage;
    }
    cum_sum
}

fn part2(banks: &Vec<String>) -> i64 {
    let mut cum_sum = 0;
    for bank in banks {
        let max_joltage = calc_max_joltage_with_x_batteries(bank, 12);
        debug!("max joltage {} for {}", max_joltage, bank);
        cum_sum += max_joltage;
    }
    cum_sum
}

fn main() {
    env_logger::init();
    
    debug!("Day 3 - Advent of Code 2025");
    
    // read input as a single string
    let input_list = read_input_lines(3).expect("Could not read input");
    //let input_list = read_sample_lines(3).expect("Could not read input");
    
    let result1 = part1(&input_list);
    let result2 = part2(&input_list);

    info!("Part 1: {}", result1);  // 16993 for real input
    info!("Part 2: {}", result2);
}
