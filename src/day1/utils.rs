use std::fs::File;
use std::io::{BufReader, Lines};

pub fn get_lists_from_input (lines: Lines<BufReader<File>>) -> (Vec<i32>, Vec<i32>) {
    let (first_list, second_list): (Vec<i32>, Vec<i32>) = lines
        .map(|line| {
            let numbers: Vec<i32> = line
                .unwrap()
                .split_whitespace()
                .map(|number| number.parse::<i32>().unwrap())
                .collect();
            (numbers[0], numbers[1])
        })
        .unzip();

    (first_list, second_list)
}