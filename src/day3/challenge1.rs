pub fn solve(input: String) -> i32 {
    let mut sum = 0;
    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(&input).for_each(|cap| {
        let a = cap[1].parse::<i32>().unwrap();
        let b = cap[2].parse::<i32>().unwrap();

        sum += a * b;
    });

    sum
}
