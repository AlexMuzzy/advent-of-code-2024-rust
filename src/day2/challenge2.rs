pub fn solve(lists: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;

    for mut list in lists {
        for _ in 0..2 { // Number of tries to remove invalid lines
            let line_result = validate_line(list.clone());

            if !line_result.result {
                list.remove(line_result.failed_at as usize);
                continue;
            }
            
            result += 1;
            break; // Break the loop if the line is valid
        }
    }

    result
}

struct LineResult {
    result: bool,
    failed_at: i32,
}

fn validate_line(line: Vec<i32>) -> LineResult {
    // Check the first two elements if they are increasing
    let is_increasing = line[0] < line[1];
    let mut line_result = LineResult {
        result: true,
        failed_at: 0,
    };
    for i in 1..line.len() {
        let diff = (line[i - 1] - line[i]).abs();

        if diff < 1 || diff > 3
            || is_increasing && line[i - 1] > line[i]
            || !is_increasing && line[i - 1] < line[i]
        {
            line_result = LineResult {
                result: false,
                failed_at: (i - 1) as i32,
            };
            break;
        }
    }
    line_result
}
