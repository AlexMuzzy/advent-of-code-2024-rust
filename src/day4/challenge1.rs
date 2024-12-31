pub fn solve(input: Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    
    for line in input {
        count += analyse_line(line);
    }
    
    count
}

fn analyse_line(line: Vec<char>) -> i32 {
    let mut count = 0;
    let mut last_char = '.';
    for char in line.clone() {
        if char == 'X' {
            last_char = 'X';
            continue;
        }
        
        if last_char == 'X' {
            if char == 'X' { 
                continue;    
            };
            
            if char == 'M' {
                last_char = 'M';
                continue;
            };
        }
        if last_char == 'M' {
            if char == 'M' {
                continue;
            }
            
            if char == 'A' {
                last_char = 'A';
                continue;
            }
        }
        if last_char == 'A' {
            if char == 'A' {
                continue;
            }
            
            if char == 'S' {
                println!("Found XMAS! count: {}", line.clone().iter().collect::<String>());                
                count += 1;
                last_char = 'S';
                continue;
            }
        }
        
        if last_char == 'S' {
            if char == 'S' {
                continue;
            }
        }
    }
    
    count
}