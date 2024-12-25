use crate::day2::utils::validate_line;

pub fn solve(lists: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for list in lists {
        if validate_line(list.clone()) {
            sum += 1;
        }
    }

    sum
}
