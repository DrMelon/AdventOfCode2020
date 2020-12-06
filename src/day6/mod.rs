// Day 6: Custom Customs
// https://adventofcode.com/2020/day/6
use super::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub fn display_day_menu(s: &mut Cursive) {
    let menu = SelectView::<i32>::new()
        .on_submit(menu_selection)
        .with_name("day_menu")
        .fixed_size((40, 15));

    s.add_layer(Dialog::around(menu).title("Day 6"));
    populate_menu(s);
}

fn populate_menu(s: &mut Cursive) {
    s.call_on_name("day_menu", |view: &mut SelectView<i32>| {
        view.add_item("1st Star ⭐", 0);
        view.add_item("2nd Star 🌟", 1);
        view.add_item("Back", 999);
    });
}

fn menu_selection(s: &mut Cursive, selection: &i32) {
    match selection {
        0 => {
            first_star(s);
        }
        1 => {
            second_star(s);
        }
        999 => {
            s.pop_layer();
        }
        _ => {}
    }
}



pub fn first_star(s: &mut Cursive) {
    // Create async dialog for this.
    let async_view = AsyncView::new_with_bg_creator(
        s,
        move || {
            let bufreader = BufReader::new(File::open("inputs/day6_1.txt").unwrap());
            let lines : Vec<String> = bufreader.lines().map(|line| line.unwrap()).collect();

            let groups = collect_groups_as_dicts(&lines);
            let total_yes_questions = calculate_total_questions_answered_any_yes(&groups);

            
            Ok(format!("Total number of groups: {}\nTotal questions answered yes: {}", groups.len(), total_yes_questions))
        },
        TextView::new,
    )
    .with_height(15)
    .with_width(30);

    s.add_layer(
        Dialog::around(async_view)
            .title("1st Star ⭐")
            .button("Neat!", |s| {
                s.pop_layer();
            }),
    );
}

pub fn second_star(s: &mut Cursive) {
    // Create async dialog for this.
    let async_view = AsyncView::new_with_bg_creator(
        s,
        move || {
            let bufreader = BufReader::new(File::open("inputs/day6_1.txt").unwrap());
            let lines : Vec<String> = bufreader.lines().map(|line| line.unwrap()).collect();

            let groups = collect_groups_as_dicts(&lines);
            let total_yes_questions = calculate_total_questions_answered_every_yes(&groups);

            
            Ok(format!("Total number of groups: {}\nTotal questions where everyone answered yes: {}", groups.len(), total_yes_questions))
        },
        TextView::new,
    )
    .with_height(15)
    .with_width(30);

    s.add_layer(
        Dialog::around(async_view)
            .title("2nd Star ⭐")
            .button("Ah, cool!", |s| {
                s.pop_layer();
            }),
    );
}

pub fn collect_groups_as_dicts(lines: &Vec<String>) -> Vec<(i32, HashMap<char, i32>)> {
    let mut hashmap_vec = Vec::new();
    let mut current_hashmap: HashMap<char, i32> = HashMap::new();
    let mut current_group_count: i32 = 0;

    lines.iter().for_each(|line| {
        if line.trim().is_empty() {
            // Close current group and move on.
            hashmap_vec.push((current_group_count, current_hashmap.clone()));
            current_group_count = 0;
            current_hashmap.clear();
        }
        else {
            current_group_count += 1;
            line.trim().chars().for_each(|chr| {
                let mut current_value: i32 = 0;
                if current_hashmap.contains_key(&chr) {
                    current_value = *current_hashmap.get(&chr).unwrap();
                }
                current_hashmap.insert(chr, current_value + 1);
            });
        }
    });

    hashmap_vec.push((current_group_count, current_hashmap.clone()));
    hashmap_vec
}

pub fn calculate_total_questions_answered_any_yes(groups: &Vec<(i32, HashMap<char, i32>)>) -> i32 {
    groups.iter().fold(0, |yescount, hashmap_tuple| yescount + hashmap_tuple.1.keys().count() as i32)
}

pub fn calculate_total_questions_answered_every_yes(groups: &Vec<(i32, HashMap<char, i32>)>) -> i32 {
    groups.iter().fold(0, |yescount, hashmap_tuple| yescount + hashmap_tuple.1.values().filter(|value_this_question| **value_this_question == hashmap_tuple.0).count() as i32 )
}

#[cfg(test)]
mod day6tests {
    use super::*;
    
    #[test]
    fn collect_groups_as_dicts_any_yes_works() {
        let test_lines = vec![
            "abc".to_string(),
            "".to_string(),
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "".to_string(),
            "ab".to_string(),
            "ac".to_string(),
            "".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "".to_string(),
            "b".to_string(),
        ];

        let groups = collect_groups_as_dicts(&test_lines);

        assert_eq!(groups.len(), 5);
        assert_eq!(groups[0].0, 1);
        assert_eq!(groups[0].1.keys().count(), 3);

        assert_eq!(groups[1].0, 3);
        assert_eq!(groups[1].1.keys().count(), 3);

        assert_eq!(groups[2].0, 2);
        assert_eq!(groups[2].1.keys().count(), 3);

        assert_eq!(groups[3].0, 4);
        assert_eq!(groups[3].1.keys().count(), 1);

        assert_eq!(groups[4].0, 1);
        assert_eq!(groups[4].1.keys().count(), 1);
    }

    #[test]
    fn calculate_total_questions_yes_works() {
        let test_lines = vec![
            "abc".to_string(),
            "".to_string(),
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "".to_string(),
            "ab".to_string(),
            "ac".to_string(),
            "".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "".to_string(),
            "b".to_string(),
        ];

        let groups = collect_groups_as_dicts(&test_lines);

        assert_eq!(calculate_total_questions_answered_any_yes(&groups), 11);
        assert_eq!(calculate_total_questions_answered_every_yes(&groups), 6);

    }
}