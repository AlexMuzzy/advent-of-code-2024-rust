use crate::day2::utils::validate_line;

pub fn solve(lists: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    // TODO: I don't like this at all as this is essentially brute forcing the solution.
    //  I attempted to refactor this to iteratively remove elements from the list and check if the
    //  list is valid but there was too many edge cases to cover.
    for list in lists {
        if !validate_line(list.clone()) {
            for i in 0..list.len() {
                let mut current_list = list.clone();
                current_list.remove(i);
                if validate_line(current_list.clone()) {
                    sum += 1;
                    break;
                }
            }
        } else {
            sum += 1; 
        }
    }
    
    sum
}
