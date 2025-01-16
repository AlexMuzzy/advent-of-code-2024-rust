use crate::day5::utils;

pub fn solve(page_ordering_rules: &[Vec<i32>], update_pages: &[Vec<i32>]) -> i32 {
    let mut count = 0;

    let rules = utils::generate_rules(&page_ordering_rules);

    for update_page in update_pages {
        let mut is_valid = true;

        let mut curr_page = update_page.clone();
        curr_page.reverse();

        for i in 0..update_page.len() - 1 {
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


