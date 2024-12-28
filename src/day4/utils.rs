use std::fs::File;
use std::io::{BufReader, Lines};

pub fn get_char_grid_from_input(input: Lines<BufReader<File>>) -> Result<Vec<Vec<char>>, std::io::Error> {
    let contents = input
        .map(|line| line.unwrap().chars().collect())
        .collect();
    Ok(contents)
}
