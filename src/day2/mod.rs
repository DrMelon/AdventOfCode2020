// Day 2: Password Philosophy
// https://adventofcode.com/2020/day/2
use super::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn display_day_menu(s: &mut Cursive) {
    let menu = SelectView::<i32>::new()
        .on_submit(menu_selection)
        .with_name("day_menu")
        .fixed_size((40, 15));

    s.add_layer(Dialog::around(menu).title("Day 2"));
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

// Iterate pairs of numbers to find a pair that sums to 2020,
// and then multiply the pair to find the answer for the first star.
pub fn first_star(s: &mut Cursive) {
    // Create async dialog for this.
    let async_view = AsyncView::new_with_bg_creator(
        s,
        move || {
            let bufreader = BufReader::new(File::open("inputs/day2_1.txt").unwrap());
            let db_entries: Vec<String> = bufreader
                .lines()
                .map(|line| {
                    return line.unwrap();
                })
                .collect();

            let valid_passwords_count = db_entries.iter().filter(|entry|  validate_entry(&parse_entry(entry))).count() as i32;

            Ok(format!("Total passwords:{}\nValid passwords: {}", db_entries.len() as i32, valid_passwords_count))
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

            Ok("Wow!")
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

pub fn parse_entry(entry: &String) -> (i32, i32, char, String) {
    // Format is as follows:
    // <int>-<int> <chr>: <password>
    // e.g 1-3 a: abcde

    // First separate the password
    let criteria_pass_split: Vec<_> = entry.split(": ").collect();
    let password = criteria_pass_split[1];
    
    // Then, the character to validate against
    let char_valid = criteria_pass_split[0].chars().last().unwrap();

    // Then split again to get the range string, then again to get the actual start and end ranges.
    let range_char_split: Vec<_> = criteria_pass_split[0].split(" ").collect();
    let range_split: Vec<i32> = range_char_split[0].split("-").collect::<Vec<&str>>().iter().map(|s| s.parse().unwrap()).collect();

    // Now we can output our parsed values.
    (range_split[0], range_split[1], char_valid, password.to_string())
}

pub fn validate_entry(entry_parsed: &(i32, i32, char, String)) -> bool {
    let target_char_count = entry_parsed.3.chars().filter(|chr| {return *chr == entry_parsed.2}).count() as i32;

    return target_char_count >= entry_parsed.0 && target_char_count <= entry_parsed.1;
}

#[cfg(test)]
mod day2tests {
    use super::*;
    
    #[test]
    fn entry_parsing_works() {
        let test_entries = vec![
            "1-3 a: abcde".to_string(),
            "1-3 b: cdefg".to_string(),
            "2-9 c: ccccccccc".to_string()];

        
        assert_eq!(day2::parse_entry(&test_entries[0]), (1, 3, 'a', "abcde".to_string()));
        assert_eq!(day2::parse_entry(&test_entries[1]), (1, 3, 'b', "cdefg".to_string()));
        assert_eq!(day2::parse_entry(&test_entries[2]), (2, 9, 'c', "ccccccccc".to_string()));
        assert_ne!(day2::parse_entry(&test_entries[2]), (0, 30, 'g', "unrelated".to_string()));
    }

    #[test]
    fn entry_validation_works() {
        let test_entries_parsed = vec![
            (1, 3, 'a', "abcde".to_string()),
            (1, 3, 'b', "cdefg".to_string()),
            (2, 9, 'c', "ccccccccc".to_string())];

        assert_eq!(day2::validate_entry(&test_entries_parsed[0]), true);
        assert_eq!(day2::validate_entry(&test_entries_parsed[1]), false);
        assert_eq!(day2::validate_entry(&test_entries_parsed[2]), true);
    }
}