use std::fs;

/// Read input file for a given day and return lines as a vector
pub fn read_input_lines(day: u8) -> Result<Vec<String>, std::io::Error> {
    let filename = format!("input/day{:02}.txt", day);
    let content = fs::read_to_string(&filename)?;
    Ok(content.lines().map(|line| line.to_string()).collect())
}

/// Read input file as a single string
pub fn read_input(day: u8) -> Result<String, std::io::Error> {
    let filename = format!("input/day{:02}.txt", day);
    fs::read_to_string(&filename)
}

/// Read sample input file and return lines as a vector
pub fn read_sample_lines(day: u8) -> Result<Vec<String>, std::io::Error> {
    let filename = format!("input/day{:02}_sample.txt", day);
    let content = fs::read_to_string(&filename)?;
    Ok(content.lines().map(|line| line.to_string()).collect())
}

/// Read sample input file as a single string
pub fn read_sample(day: u8) -> Result<String, std::io::Error> {
    let filename = format!("input/day{:02}_sample.txt", day);
    fs::read_to_string(&filename)
}
