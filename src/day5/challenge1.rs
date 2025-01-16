use std::collections::HashMap;

pub fn solve(page_ordering_rules: &[Vec<i32>], update_pages: &[Vec<i32>]) -> i32 {
    let mut count = 0;

    let rules: HashMap<&i32, Vec<i32>> = generate_rules(page_ordering_rules);
    
    for update_page in update_pages {
        let mut is_valid = true;

        let mut curr_page = update_page.clone();
        curr_page.reverse();

        for i in 0..curr_page.len() - 1 {
            let rule = rules.get(&curr_page[i]);
            match rule {
                Some(rule) => {
                    for j in i + 1..curr_page.len() {
                        if rule.contains(&curr_page[j]) {
                            is_valid = false;
                            break;
                        }
                    }
                }
                None => continue,
            }
        }

        if is_valid {
            // obtain the middle index
            let mid = curr_page.len() / 2;
            count += curr_page[mid];
        }
    }
    
    count
}

fn generate_rules(page_ordering_rules: &[Vec<i32>]) -> HashMap<&i32, Vec<i32>> {
    page_ordering_rules.iter().fold(HashMap::new(), |mut acc, rule| {
        if acc.contains_key(&rule[0]) {
            acc.get_mut(&rule[0]).unwrap().push(rule[1]);
        } else {
            acc.insert(&rule[0], vec![rule[1]]);
        }
        acc
    })
}
