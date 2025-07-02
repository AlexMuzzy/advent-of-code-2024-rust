use crate::day5::utils;

pub fn solve(page_ordering_rules: &[Vec<i32>], update_pages: &[Vec<i32>]) -> i32 {
    let mut count = 0;
    let rules = utils::generate_rules(&page_ordering_rules);
    for update_page in update_pages {
        if !utils::validate_rule(&rules, &update_page) {
            let reordered_page = utils::reorder_invalid_page(&rules, &mut update_page.clone());
            let middle_index = reordered_page.len() / 2;
            count += reordered_page[middle_index];
        }
    }

    count
}
