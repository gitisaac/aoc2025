use aoc2025::*;
use log::debug;

const MOD_STEP: i32 = 100;
const START_STEP: i32 = 50;

#[derive(Debug)]
enum ProcessError {
    InvalidDirection,
    ParseError(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for ProcessError {
    fn from(err: std::num::ParseIntError) -> Self {
        ProcessError::ParseError(err)
    }
}

fn calculate_new_position(current_step: i32, delta: i32) -> i32 {
    let new_step = current_step + delta;
    let remainder = new_step % MOD_STEP;
    if remainder < 0 {
        remainder + MOD_STEP
    } else {
        remainder
    }
}

fn process_line(line: &String) -> Result<i32, ProcessError> {
    // each line is expected to first have either an L or R character
    // L means turn left, R means turn right, followed by a number 
    // indicating steps to move forward.
    let (turn, steps_str) = line.trim().split_at(1);
    let steps: i32 = steps_str.parse()?;  // this will auto-convert ParseIntError to ProcessError
    match turn {
        "L" => {
            Ok(-steps)
        },
        "R" => {
            Ok(steps)
        },
        _ => {
            Err(ProcessError::InvalidDirection)
        }
    }
}

fn part1(input: &Vec<String>) -> Result<i32, ProcessError> {
    if !input.is_empty() {
        let mut times_at_zero = 0;
        let mut current_step = START_STEP;
        for line in input {
            debug!("Processing line: {}, and current step: {}", line, current_step);
            let delta = process_line(line)?;
            current_step = calculate_new_position(current_step, delta);
            if current_step == 0 {
                times_at_zero += 1;
            }
        }
        Ok(times_at_zero)
    } else {
        Ok(0)
    }
}

fn part2(input: &Vec<String>) -> Result<i32, ProcessError> {
    if !input.is_empty() {
        let mut times_visited_zero = 0;
        let mut current_step = START_STEP;
        for line in input {
            debug!("Processing line: {}, and current step: {}", line, current_step);
            let delta = process_line(line)?;
            let raw_step = current_step + delta;
            let laps = (raw_step / MOD_STEP).abs();
            debug!("raw_step / mod_step: {} / {} = {}", raw_step, MOD_STEP, laps);
            times_visited_zero += laps;
            let passed_zero_backwards = current_step > 0 && raw_step < 0;
            times_visited_zero += passed_zero_backwards as i32;

            current_step = calculate_new_position(current_step, delta);
            if current_step == 0 {
                times_visited_zero += 1;
                if laps > 0 {
                    times_visited_zero -= 1;
                }
            }
        }
        Ok(times_visited_zero)
    } else {
        Ok(0)
    }
}

fn main() {
    env_logger::init();

    println!("Day 1 - Advent of Code 2025");

    let lines = read_input_lines(1).expect("Could not read input file");
    //let lines = read_sample_lines(1).expect("Could not read input file");

    let result1 = part1(&lines).expect("Could not parse lines!");
    let result2 = part2(&lines).expect("Could not parse lines!");

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
}
