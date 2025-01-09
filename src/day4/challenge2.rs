pub fn solve(input: &[Vec<char>]) -> i32 {
    let mut count = 0;

    for i in 1..input.len() - 1 {
        for j in 1..input[0].len() - 1 {
            if input[i][j] == 'A' {
                if verify_valid_map(input, &i, &j) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn verify_valid_map(input: &[Vec<char>], x: &usize, y: &usize) -> bool {
    let combinations = [
        ('M', 'M', 'S', 'S'),
        ('M', 'S', 'S', 'M'),
        ('S', 'S', 'M', 'M'),
        ('S', 'M', 'M', 'S'),
    ];

    for (a, b, c, d) in combinations.iter() {
        if input[x - 1][y - 1] == *a &&
            input[x - 1][y + 1] == *b &&
            input[x + 1][y + 1] == *c &&
            input[x + 1][y - 1] == *d {
            return true;
        }
    }

    false
}