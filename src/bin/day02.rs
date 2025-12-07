use aoc2025::*;
use log::{debug, info};

fn count_digits(n: i64) -> u32 {
    match n {
        0 => 1,
        n => n.abs().ilog10() + 1
    }
}

fn is_invalid_id(n: i64) -> bool {
    let n_str: String = n.to_string();
    let (first, last) = n_str.split_at(n_str.len()/2);
    first == last
}

fn are_all_elements_equal<T: PartialEq>(vec: &Vec<T>) -> bool {
    // Empty or single-element vectors have all elements equal (vacuously true/trivially true)
    if vec.len() < 2 {
        return true;
    }
    // Get the first element
    let first = &vec[0];
    // Check if all subsequent elements are equal to the first
    vec.iter().skip(1).all(|item| item == first)
}

fn is_invalid_id_part_2(n: i64) -> bool {
    let n_str: String = n.to_string();
    let middle_i = n_str.len() / 2;
    for i in 1..middle_i + 1 {
        let subs = n_str.as_bytes()
            .chunks(i)
            .map(str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap();
        if are_all_elements_equal(&subs) {
            return true
        }
    }
    false
}

fn has_even_digit_count(n: i64) -> bool {
    if n == 0 { return false; } // 0 has 1 digit (odd)
    
    let digit_count = n.abs().ilog10() + 1;
    digit_count % 2 == 0
}

fn next_even_digit_number(n: i64) -> i64 {
    let current_digits = count_digits(n);
    
    match current_digits % 2 {
        1 => {
            // Odd digits: jump to smallest number with (current_digits + 1) digits
            // Examples: 777 (3 digits) -> 1000 (4 digits)
            //          99999 (5 digits) -> 100000 (6 digits)
            10_i64.pow(current_digits)
        },
        _ => n + 1, // Already even digits, just increment
    }
}

fn find_invalid_ids_in_range(start: i64, stop: i64) -> Vec<i64> {
    let mut invalid_ids: Vec<i64> = Vec::new();
    let mut i = start;

    while i <= stop {
        if has_even_digit_count(i) {
            if is_invalid_id(i) {
                invalid_ids.push(i);
            }
            i += 1;
        } else {
            i = next_even_digit_number(i);
        }
    }
    invalid_ids
}

fn find_invalid_ids_in_range_part_2(start: i64, stop: i64) -> Vec<i64> {
    let mut invalid_ids: Vec<i64> = Vec::new();
    let mut i = start;

    while i <= stop {
        if is_invalid_id_part_2(i) {
            invalid_ids.push(i);
        }
        i += 1;
    }
    invalid_ids
}

fn part1(ranges: &Vec<&str>) -> i64 {
    let mut cum_sum: i64 = 0;
    for range in ranges {
        let (start, stop) = range.split_once('-').expect(&format!("Could not parse range {}", range));
        let start_number: i64 = start.parse().expect("Could not parse number from start number of range");
        let stop_number: i64 = stop.parse().expect("Could not parse number from start number of range");
        let sum_of_invalid_ids = find_invalid_ids_in_range(start_number, stop_number).iter().sum::<i64>();
        debug!("sum of invalid ids for {} = {}", range, sum_of_invalid_ids);
        cum_sum += sum_of_invalid_ids
    }
    cum_sum
}

fn part2(ranges: &Vec<&str>) -> i64 {
    let mut cum_sum: i64 = 0;
    for range in ranges {
        let (start, stop) = range.split_once('-').expect(&format!("Could not parse range {}", range));
        let start_number: i64 = start.parse().expect("Could not parse number from start number of range");
        let stop_number: i64 = stop.parse().expect("Could not parse number from start number of range");
        let sum_of_invalid_ids = find_invalid_ids_in_range_part_2(start_number, stop_number).iter().sum::<i64>();
        debug!("sum of invalid ids for {} = {}", range, sum_of_invalid_ids);
        cum_sum += sum_of_invalid_ids
    }
    cum_sum
}

fn main() {
    env_logger::init();
    
    debug!("Day 2 - Advent of Code 2025");
    
    // read input as a single string
    let input = read_input(2).expect("Could not read input");
    //let input = read_sample(2).expect("Could not read input");

    // split on comma ','
    let input_list = input.trim().split(',').collect::<Vec<&str>>();
    
    let result1 = part1(&input_list);
    let result2 = part2(&input_list);

    info!("Part 1: {}", result1);
    info!("Part 2: {}", result2);
}
