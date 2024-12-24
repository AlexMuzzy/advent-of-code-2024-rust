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