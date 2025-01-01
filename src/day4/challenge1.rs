pub fn solve(input: Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    
    for line in input.clone() { // check each line horizontally
        count += analyse_line(line.clone());
        
        // check again in reverse
        let mut reversed_line = line.clone();
        reversed_line.reverse();
        count += analyse_line(reversed_line);
    }
    
    for i in 0..input[0].clone().len() { // check each line vertically
        let mut line = Vec::new();
        for j in 0..input.len() {
            line.push(input[j][i]);
        }
        count += analyse_line(line.clone());
        
        // check again in reverse
        let mut reversed_line = line.clone();
        reversed_line.reverse();
        count += analyse_line(reversed_line);
    }
    
    for i in 0..input.len() { // check each line diagonally - top left to bottom right
        let mut line = Vec::new();
        for j in 0..input.len() {
            if i + j < input.len() {
                line.push(input[j][i + j]);
            }
        }
        count += analyse_line(line.clone());
        
        // check again in reverse
        let mut reversed_line = line.clone();
        reversed_line.reverse();
        count += analyse_line(reversed_line);
    }
    
    for i in input.len() - 1..0 { // check each line diagonally - bottom left to top right
        let mut line = Vec::new();
        for j in 0..input.len() {
            if i - j >= 0 {
                line.push(input[j][i - j]);
            }
        }
        count += analyse_line(line.clone());
        
        // check again in reverse
        let mut reversed_line = line.clone();
        reversed_line.reverse();
        count += analyse_line(reversed_line);
    }
    
    count
}

fn analyse_line(line: Vec<char>) -> i32 {
    let mut count = 0;
    let mut last_char = '.';

    for char in line.clone() {
        match (last_char, char) {
            ('X', 'M') => last_char = 'M',
            ('M', 'A') => last_char = 'A',
            ('A', 'S') => {
                count += 1;
                last_char = 'S';
                println!("Found XMAS! count: {}", line.iter().collect::<String>());
            },
            ('X', 'X') | ('M', 'M') | ('A', 'A') | ('S', 'S') => {},
            ('X', _) | ('M', _) | ('A', _) | ('S', _) => last_char = '.',
            (_, 'X') => last_char = 'X',
            _ => last_char = '.',
        }
    }

    count
}