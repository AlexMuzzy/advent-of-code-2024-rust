use crate::day5::utils;

pub fn solve(page_ordering_rules: &[Vec<i32>], update_pages: &[Vec<i32>]) -> i32 {
    let mut count = 0;
    let rules = utils::generate_rules(&page_ordering_rules);

    for update_page in update_pages {
        if !utils::validate_rule(&rules, &update_page) {
            let mut is_valid = false;
            while !is_valid {
                for i in (0..update_page.len()).rev() {
                    let rule = rules.get(&update_page[i]);
                    match rule {
                        None => continue,
                        Some(rule) => {
                            for j in (1..rules.len()).rev() {
                                if rule.contains(&update_page[j]) {
                                    
                                }
                            }
                        }
                    }
                }
                
                is_valid = true;
            }
        }
    }

    count
}