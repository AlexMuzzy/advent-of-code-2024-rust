use std::fs::File;
use std::io::{BufReader, Lines};

pub fn get_lists_from_input (lines: Lines<BufReader<File>>) -> Vec<Vec<i32>> {
    lines.map(|line| {
        line
            .unwrap()
            .split_whitespace()
            .map(|number| number.parse::<i32>().unwrap())
            .collect()
    }).collect()
}

pub fn validate_line(line: Vec<i32>) -> bool {
    let mut result = true;
    // Check the first two elements if they are increasing
    let is_increasing = line[0] < line[1];
    for i in 1..line.len() {
        let diff = (line[i - 1] - line[i]).abs();

        if diff < 1
            || diff > 3
            || is_increasing && line[i - 1] > line[i]
            || !is_increasing && line[i - 1] < line[i]
        {
            result = false;
            break;
        }
    }
    result
}
