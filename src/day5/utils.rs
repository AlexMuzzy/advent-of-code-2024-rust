use std::collections::HashMap;
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
            let rule = line.split("|").map(|s| s.parse::<i32>().unwrap()).collect();
            ordering_rules.push(rule);
        } else {
            let page = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
            update_pages.push(page);
        }
    }

    Ok((ordering_rules, update_pages))
}

pub fn reorder_invalid_page(
    rules: &HashMap<&i32, Vec<i32>>,
    update_page: &mut Vec<i32>,
) -> Vec<i32> {
    let mut reordered = update_page.clone();
    let n = reordered.len();
    let mut changed = true;

    while changed {
        changed = false;
        for i in 0..n {
            for j in i + 1..n {
                if let Some(afters) = rules.get(&reordered[i]) {
                    if afters.contains(&reordered[j]) {
                        reordered.swap(i, j);
                        changed = true;
                    }
                }
            }
        }
    }
    reordered
}

pub fn generate_rules(page_ordering_rules: &[Vec<i32>]) -> HashMap<&i32, Vec<i32>> {
    page_ordering_rules
        .iter()
        .fold(HashMap::new(), |mut acc, rule| {
            if acc.contains_key(&rule[0]) {
                acc.get_mut(&rule[0]).unwrap().push(rule[1]);
            } else {
                acc.insert(&rule[0], vec![rule[1]]);
            }
            acc
        })
}

pub fn validate_rule(rules: &HashMap<&i32, Vec<i32>>, update_page: &[i32]) -> bool {
    update_page.iter().enumerate().rev().all(|(i, &page)| {
        rules.get(&page).map_or(true, |rule| {
            update_page[..i].iter().all(|next| !rule.contains(next))
        })
    })
}
