pub fn solve((first_list, second_list): (Vec<i32>, Vec<i32>)) -> i32 {
    let mut result = 0;

    for i in 0..first_list.len() {
        let frequency = second_list.iter().filter(|&x| *x == first_list[i]).count();
        result += first_list[i] * frequency as i32;
    }

    result
}
