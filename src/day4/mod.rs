// Day 4: Passport Processing
// https://adventofcode.com/2020/day/4
use super::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

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
            
            let valid_passport_count = processed_lines.iter().filter(|line| convert_processed_line(line, false).is_some()).count();
            
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
            let unprocessed_lines: Vec<String> = bufreader.lines().map(|line| line.unwrap()).collect();

            let processed_lines = preprocess_strip_blank_lines(&unprocessed_lines);
            
            let valid_passport_count = processed_lines.iter().filter(|line| convert_processed_line(line, true).is_some()).count();
            
            Ok(format!("Passports scanned: {}\nPassports valid: {}", processed_lines.len(), valid_passport_count))
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
    cid: Option<String>,
    star2_format: bool
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
            cid: None,
            star2_format: false
            }
    }

    fn is_valid(&self) -> bool {
        match self.star2_format {
            true => self.is_valid_star2(),
            false => self.is_valid_star1()
        }
    }

    fn is_valid_star1(&self) -> bool {
        self.byr.is_some() &&
        self.iyr.is_some() &&
        self.eyr.is_some() &&
        self.hgt.is_some() &&
        self.hcl.is_some() &&
        self.ecl.is_some() &&
        self.pid.is_some()
    }

    fn is_valid_star2(&self) -> bool {
        if !self.is_valid_star1() {
            return false;
        }

        let mut valid = true;      
        
        // Birth year is a 4 digit number between 1920 and 2002.
        valid &= self.byr.as_ref().unwrap().len() == 4;
        if self.byr.as_ref().unwrap().parse::<i32>().is_ok() {
            let byr = self.byr.as_ref().unwrap().parse::<i32>().unwrap();
            valid &= byr >= 1920 && byr <= 2002;
        }
        else {
            valid = false;
        }


        // Issue year is a 4 digit number between 2010 and 2020.
        valid &= self.iyr.as_ref().unwrap().len() == 4;
        if self.iyr.as_ref().unwrap().parse::<i32>().is_ok() {
            let iyr = self.iyr.as_ref().unwrap().parse::<i32>().unwrap();
            valid &= iyr >= 2010 && iyr <= 2020;
        }
        else {
            valid = false;
        }

        // Expiration year is a 4 digit number between 2020 and 2030.
        valid &= self.eyr.as_ref().unwrap().len() == 4;
        if self.eyr.as_ref().unwrap().parse::<i32>().is_ok() {
            let eyr = self.eyr.as_ref().unwrap().parse::<i32>().unwrap();
            valid &= eyr >= 2020 && eyr <= 2030;
        }
        else {
            valid = false;
        }

        // Height is a number followed by "cm" or "in".
        let height_pattern = Regex::new(r"([0-9]+)(cm|in)").unwrap();
        let height_caps = height_pattern.captures(self.hgt.as_ref().unwrap());
        if height_caps.is_some() {
            let height_num = height_caps.as_ref().unwrap().get(1);
            let height_unit = height_caps.as_ref().unwrap().get(2);
            
            valid &= height_num.is_some() && height_unit.is_some();
            if valid {
                valid &= height_num.as_ref().unwrap().as_str().parse::<i32>().is_ok();
                if valid {
                    let height_numeric = height_num.as_ref().unwrap().as_str().parse::<i32>().unwrap();

                    match height_unit.unwrap().as_str() {
                        "cm" => {
                            valid &= height_numeric >= 150 && height_numeric <= 193;
                        },
                        "in" => {
                            valid &= height_numeric >= 59 && height_numeric <= 76;
                        },
                        _ => { valid = false; }
                    }
                }
            }
        }
        else {
            valid = false;
        }

        // Hair color is a # followed by six characters 0-9 or a-f
        let hair_pattern = Regex::new(r"#([0-9 a-f]{6})").unwrap();
        valid &= self.hcl.as_ref().unwrap().len() == 7;
        valid &= hair_pattern.is_match(self.hcl.as_ref().unwrap());

        // Eye color is exactly one of: amb blu brn gry grn hzl oth
        let eye_pattern = Regex::new(r"(amb|blu|brn|gry|grn|hzl|oth)").unwrap();
        valid &= eye_pattern.is_match(self.ecl.as_ref().unwrap());
        valid &= eye_pattern.find_iter(self.ecl.as_ref().unwrap()).count() == 1;

        // Passport id is a 9-digit number.
        let pid_pattern = Regex::new(r"[0-9]{9}").unwrap();
        valid &= self.pid.as_ref().unwrap().len() == 9;
        valid &= pid_pattern.is_match(self.pid.as_ref().unwrap());

        valid
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

pub fn convert_processed_line(line: &String, star2_format: bool) -> Option<Passport> {

    let mut passport = Passport::new();
    passport.star2_format = star2_format;
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
    fn passport_validation_works_star1() {
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
            star2_format: false
        };
        let valid_north_pole_id = Passport {
            byr: Some("a".to_string()),
            iyr: Some("a".to_string()),
            eyr: Some("a".to_string()),
            hgt: Some("a".to_string()),
            hcl: Some("a".to_string()),
            ecl: Some("🎄".to_string()),
            pid: Some("a".to_string()),
            cid: None,
            star2_format: false
        };


        assert_eq!(invalid_passport.is_valid(), false);
        assert_eq!(valid_passport.is_valid(), true);
        assert_eq!(valid_north_pole_id.is_valid(), true);
    }

    #[test]
    fn passport_validation_works_star2() {
        let invalid_passports = vec![
            "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926".to_string(),
            "iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946".to_string(),
            "hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277".to_string(),
            "hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007".to_string()
        ];

        assert_eq!(invalid_passports.iter().filter(|pass| day4::convert_processed_line(pass, true).is_some()).count(), 0);
    }

    #[test]
    fn line_conversion_works() {
        let test_line = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm".to_string();

        let passport = day4::convert_processed_line(&test_line, false);
        assert_eq!(passport.is_some(), true);
    }
}