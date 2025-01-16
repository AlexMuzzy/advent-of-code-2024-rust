#![allow(dead_code)]

// Gave up with this impl, below is ChatGPTs finest work
pub fn solve(input: Vec<Vec<char>>) -> i32 {
    let mut count = 0;

    println!("Checking {}x{} grid", input.len(), input[0].len());

    println!("Checking horizontally");
    for i in 0..input.len() {
        // check each line horizontally
        let mut line = input[i].clone();
        count += analyse_line(&line);

        // check again in reverse
        line.reverse();
        count += analyse_line(&line);
    }

    println!("Checking vertically");
    for i in 0..input[0].clone().len() {
        // check each line vertically
        let mut line = Vec::new();
        for j in 0..input.len() {
            line.push(input[j][i]);
        }
        count += analyse_line(&line);

        // check again in reverse
        line.reverse();
        count += analyse_line(&line);
    }

    println!("Checking diagonally - top left to bottom right");
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let mut line = Vec::new();
            let mut x = i;
            let mut y = j;
            while x < input.len() && y < input[0].len() {
                line.push(input[x][y]);
                x += 1;
                y += 1;
            }
            count += analyse_line(&line);
            line.reverse();
            count += analyse_line(&line);
        }
    }

    println!("Checking diagonally - bottom left to top right");
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let mut line = Vec::new();
            let mut x = i;
            let mut y = j;
            while x < input.len() && y > 0 {
                line.push(input[x][y]);
                x += 1;
                y -= 1;
            }
            count += analyse_line(&line);
            line.reverse();
            count += analyse_line(&line);
        }
    }

    count
}

pub fn count_xmas(grid: &[Vec<char>]) -> usize {
    let mut count = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    for i in 0..rows {
        for j in 0..cols {
            count += check_xmas(grid, i, j, rows, cols);
        }
    }

    count
}

fn check_xmas(grid: &[Vec<char>], row: usize, col: usize, rows: usize, cols: usize) -> usize {
    let mut count = 0;
    let directions = [
        (1, 0),  // Horizontal
        (0, 1),  // Vertical
        (1, 1),  // Diagonal down-right
        (1, -1), // Diagonal down-left
    ];

    for &(dr, dc) in &directions {
        for &reverse in &[false, true] {
            let mut r = row;
            let mut c = col;
            let mut word = String::new();

            for _ in 0..4 {
                if r < rows && c < cols {
                    word.push(grid[r][c]);
                }
                r = (r as i32 + dr) as usize;
                c = (c as i32 + dc) as usize;
            }

            if reverse {
                word = word.chars().rev().collect();
            }

            if word == "XMAS" {
                count += 1;
            }
        }
    }

    count
}

fn analyse_line(line: &Vec<char>) -> i32 {
    let mut score = 0;
    let mut count = 0;

    for char in line {
        match (score, char) {
            (1, 'M') => score = 2,
            (2, 'A') => score = 3,
            (3, 'S') => {
                score = 0;
                count += 1;
                println!("Found XMAS! count: {}", line.iter().collect::<String>());
            }
            (_, 'X') => score = 1, // Treat X as a new start
            (_, _) => score = 0,   // This catches all other combinations
        }
    }

    count
}
