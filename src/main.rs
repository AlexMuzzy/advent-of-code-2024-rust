// Act as the entry point of the program for the advent of code submissions

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() -> io::Result<()> {
    let day1_input = day1::utils::get_lists_from_input(read_lines(
        "./src/inputs/day1-input1.txt",
    )?);
    println!("Day 1 - Part 1: {}", day1::challenge1::solve(day1_input.clone()));
    println!("Day 1 - Part 2: {}", day1::challenge2::solve(day1_input.clone()));

    let day2_input = day2::utils::get_lists_from_input(read_lines(
        "./src/inputs/day2-input1.txt",
    )?);

    println!("Day 2 - Part 1: {}", day2::challenge1::solve(day2_input.clone()));
    println!("Day 2 - Part 2: {}", day2::challenge2::solve(day2_input.clone()));

    let day3_input = day3::utils::get_string_from_file(
        "./src/inputs/day3-input1.txt"
    )?;

    println!("Day 3 - Part 1: {}", day3::challenge1::solve(day3_input.clone()));
    println!("Day 3 - Part 2: {}", day3::challenge2::solve(day3_input.clone()));
    
    let day4_input = day4::utils::get_char_grid_from_input(read_lines(
        "./src/inputs/day4-input1.txt"
    )?)?;
    
    println!("Day 4 - Part 1: {}", day4::challenge1::count_xmas(&day4_input));
    println!("Day 4 - Part 2: {}", day4::challenge2::solve(&day4_input));

    let day5_input = day5::utils::get_ordering_rules_and_update_pages_from_input(read_lines(
        "./src/inputs/day5-input1.txt"
    )?)?;

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
