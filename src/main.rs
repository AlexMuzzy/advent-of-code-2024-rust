// Act as the entry point of the program for the advent of code submissions

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

mod day1;

fn main() -> io::Result<()> {
    let day1_input = day1::utils::get_lists_from_input(read_lines(
        "/home/alex/dev/advent-of-code-2024-rust/src/day1/input1.txt",
    )?);
    println!("Day 1 - Part 1: {}", day1::challenge1::solve(day1_input.clone()));
    println!("Day 1 - Part 2: {}", day1::challenge2::solve(day1_input.clone()));

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
