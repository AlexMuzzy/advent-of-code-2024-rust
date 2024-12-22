pub fn solve((first_list, second_list): (Vec<i32>, Vec<i32>)) -> i32 {
    let mut first_list = first_list; // Copy the list to sort it
    let mut second_list = second_list;

    first_list.sort();
    second_list.sort();

    let mut result = 0;

    for i in 0..first_list.len() {
        result += (first_list[i] - second_list[i]).abs()
    }

    result
}
