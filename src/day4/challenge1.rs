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
    // check each line diagonally - top left to bottom right
    for i in 0..input.len() {
        let mut column_line = Vec::new();
        let mut row_line = Vec::new();
        for j in 0..input.len() {
            if i + j < input.len() {
                println!("Adding {} to column_line, x={}, y={}", input[j][i + j], j, i + j);
                column_line.push(input[j][i + j]);
            }
        }

        for j in 1..input.len() {
            if i + j < input.len() {
                // skip the centre line as it's already been checked
                row_line.push(input[i + j][j]);
            }
        }
        count += analyse_line(&column_line);
        count += analyse_line(&row_line);

        // check again in reverse
        column_line.reverse();
        count += analyse_line(&column_line);

        row_line.reverse();
        count += analyse_line(&row_line);
    }

    println!("Checking diagonally - bottom left to top right");
    for i in (0..input.len()).rev() {
        let mut column_line = Vec::new();
        let mut row_line = Vec::new();
        for j in 0..input.len() {
            let ii = i as i32;
            let jj = j as i32;
            if ii - jj >= 0 {
                println!("Adding {} to column_line, x={}, y={}", input[i - j][j], i - j, j);
                column_line.push(input[i - j][j]);
            }
        }

        for j in 1..input.len() {
            let ii = i as i32;
            let jj = j as i32;
            if ii - jj >= 0 {
                // skip the centre line as it's already been checked
                row_line.push(input[i - j][j]);
            }
        }

        count += analyse_line(&column_line);
        count += analyse_line(&row_line);

        // check again in reverse
        column_line.reverse();
        count += analyse_line(&column_line);

        row_line.reverse();
        count += analyse_line(&row_line);
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
