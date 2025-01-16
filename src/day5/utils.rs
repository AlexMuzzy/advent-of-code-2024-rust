use std::{
    fs::File,
    io::{BufReader, Lines},
};

pub fn get_ordering_rules_and_update_pages_from_input(
    input: Lines<BufReader<File>>,
) -> Result<(Vec<Vec<i32>>, Vec<Vec<i32>>), std::io::Error> {
    let mut ordering_rules = Vec::new();
    let mut update_pages = Vec::new();
    let mut is_ordering_rules = true;

    for line in input {
        let line = line?;
        if line.is_empty() {
            is_ordering_rules = false;
            continue;
        }

        if is_ordering_rules {
            let rule = line
                .split("|")
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            ordering_rules.push(rule);
        } else {
            let page = line
                .split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            update_pages.push(page);
        }
    }

    Ok((ordering_rules, update_pages))
}
