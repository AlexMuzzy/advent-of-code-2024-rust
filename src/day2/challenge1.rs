pub fn solve(lists: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    
    for list in lists {
        // Check the first two elements if they are increasing
        let is_increasing = list[0] < list[1];
        let mut is_valid = true;
        for i in 1..list.len() {
            let diff = (list[i-1] - list[i]).abs();
            if diff < 1 || diff > 3 {
                is_valid = false;
                break;
            }
            
            if is_increasing && list[i-1] > list[i] {
                is_valid = false;
                break;
            }
            if !is_increasing && list[i-1] < list[i] {
                is_valid = false;
                break;
            }
        }
        if is_valid {
            result += 1;
        }
    }
    
    result
}