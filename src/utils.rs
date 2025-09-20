use std::time::Instant;
use std::{fs::File, io, io::BufRead, path::Path};

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn print_and_measure_execution_time(prefix_string: &str, func: impl Fn() -> i32) {
    let now = Instant::now();
    let result = func();
    let elapsed = now.elapsed().as_millis();
    println!("{prefix_string}: {result} | Time taken: {elapsed:?}ms");
}
