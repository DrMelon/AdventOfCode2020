// Day 4: Passport Processing
// https://adventofcode.com/2020/day/4
use super::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn display_day_menu(s: &mut Cursive) {
    let menu = SelectView::<i32>::new()
        .on_submit(menu_selection)
        .with_name("day_menu")
        .fixed_size((40, 15));

    s.add_layer(Dialog::around(menu).title("Day 4"));
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
            let bufreader = BufReader::new(File::open("inputs/day4_1.txt").unwrap());
            let unprocessed_lines: Vec<String> = bufreader.lines().map(|line| line.unwrap()).collect();

            let processed_lines = preprocess_strip_blank_lines(&unprocessed_lines);
            
            let valid_passport_count = processed_lines.iter().filter(|line| convert_processed_line(line).is_some()).count();
            
            Ok(format!("Passports scanned: {}\nPassports valid: {}", processed_lines.len(), valid_passport_count))
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
            let bufreader = BufReader::new(File::open("inputs/day4_1.txt").unwrap());
            
            Ok(format!("Yup!"))
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

pub struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>
}

impl Passport {
    fn new() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None
            }
    }

    fn is_valid(&self) -> bool {
        self.byr.is_some() &&
        self.iyr.is_some() &&
        self.eyr.is_some() &&
        self.hgt.is_some() &&
        self.hcl.is_some() &&
        self.ecl.is_some() &&
        self.pid.is_some()
    }
}

pub fn preprocess_strip_blank_lines(lines: &Vec<String>) -> Vec<String> {
    // Generate a new set of lines, with one passport per.
    let mut processed_lines = Vec::new();

    let mut current_line = "".to_string();
    for line in lines {
        if !line.trim().is_empty() {
            if !current_line.is_empty()
            {
                current_line.push(' ');
            }
            current_line.push_str(line.trim());
        }
        else {
            // empty line found, push this line back to the processed lines and start over
            processed_lines.push(current_line.to_string());
            current_line = "".to_string();
        }
    }

    // Push final line if not empty.
    if !current_line.is_empty() {
        processed_lines.push(current_line.to_string());
    }

    processed_lines
}

pub fn convert_processed_line(line: &String) -> Option<Passport> {

    let mut passport = Passport::new();
    let key_pairs: Vec<&str> = line.split(' ').collect();
    key_pairs.iter().for_each(|kvp| {
        let mut split_iter = kvp.split(':');
        let key = split_iter.next().unwrap();
        let value = split_iter.next().unwrap();

        match key {
            "byr" => passport.byr = Some(value.to_string()),
            "iyr" => passport.iyr = Some(value.to_string()),
            "eyr" => passport.eyr = Some(value.to_string()),
            "hgt" => passport.hgt = Some(value.to_string()),
            "hcl" => passport.hcl = Some(value.to_string()),
            "ecl" => passport.ecl = Some(value.to_string()),
            "pid" => passport.pid = Some(value.to_string()),
            "cid" => passport.cid = Some(value.to_string()),
            _=> {}
        }
    });

    match passport.is_valid() {
        true => Some(passport),
        false => None
    }
}


#[cfg(test)]
mod day4tests {
    use super::*;
    
    #[test]
    fn preprocess_works() {
        let test_data = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string(),
            "byr:1937 iyr:2017 cid:147 hgt:183cm".to_string(),
            "".to_string(),
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884".to_string(),
            "hcl:#cfa07d byr:1929".to_string(),
            "".to_string(),
            "hcl:#ae17e1 iyr:2013".to_string(),
            "eyr:2024".to_string(),
            "ecl:brn pid:760753108 byr:1931".to_string(),
            "hgt:179cm".to_string(),
            "".to_string(),
            "hcl:#cfa07d eyr:2025 pid:166559648".to_string(),
            "iyr:2011 ecl:brn hgt:59in".to_string()
        ];

        let processed_lines = day4::preprocess_strip_blank_lines(&test_data);
        assert_eq!(processed_lines.len(), 4);
        assert_eq!(processed_lines[0].len(), 78);
    }

    #[test]
    fn passport_validation_works() {
        let invalid_passport = Passport::new();
        let valid_passport = Passport { 
            byr: Some("a".to_string()),
            iyr: Some("a".to_string()),
            eyr: Some("a".to_string()),
            hgt: Some("a".to_string()),
            hcl: Some("a".to_string()),
            ecl: Some("a".to_string()),
            pid: Some("a".to_string()),
            cid: Some("a".to_string()),
        };
        let valid_north_pole_id = Passport {
            byr: Some("a".to_string()),
            iyr: Some("a".to_string()),
            eyr: Some("a".to_string()),
            hgt: Some("a".to_string()),
            hcl: Some("a".to_string()),
            ecl: Some("🎄".to_string()),
            pid: Some("a".to_string()),
            cid: None
        };

        assert_eq!(invalid_passport.is_valid(), false);
        assert_eq!(valid_passport.is_valid(), true);
        assert_eq!(valid_north_pole_id.is_valid(), true);

    }

    #[test]
    fn line_conversion_works() {
        let test_line = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm".to_string();

        let passport = day4::convert_processed_line(&test_line);
        assert_eq!(passport.is_some(), true);
    }
}