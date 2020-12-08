// Day 7: Handy Haversacks
// https://adventofcode.com/2020/day/7
use super::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::*;
use itertools::Itertools;
use std::collections::HashMap;

pub fn display_day_menu(s: &mut Cursive) {
    let menu = SelectView::<i32>::new()
        .on_submit(menu_selection)
        .with_name("day_menu")
        .fixed_size((40, 15));

    s.add_layer(Dialog::around(menu).title("Day 7"));
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
            // Load input file and parse it into a vec of ints
            let bufreader = BufReader::new(File::open("inputs/day7.txt").unwrap());
            let lines: Vec<String> = bufreader.lines().map(|line| line.unwrap()).collect();
            let bags = find_all_kinds_of_bag(&lines);
            let rules = find_all_rules(&lines);
            
            Ok(format!("Wow! There are {} bags that eventually contain the shiny gold bag.", count_bags_that_contain_bag("shiny gold".to_string(), &bags, &rules)))
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
            // Load input file and parse it into a vec of ints
            let bufreader = BufReader::new(File::open("inputs/day7.txt").unwrap());

            Ok(format!("Yeah!"))
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

pub fn find_all_kinds_of_bag(lines: &Vec<String>) -> Vec<String> {
    let mut bags: Vec<String> = Vec::new();

    let bag_pattern = regex::Regex::new(r"(\w+) (\w+) bag").unwrap();
    
    lines.iter().for_each(|line| {
        for bag_capture in bag_pattern.captures_iter(line) {
            bags.push(bag_capture[1].to_string() + " " + &bag_capture[2].to_string());
        }
    });

    bags.iter().unique().map(|bag| bag.to_string()).collect::<Vec<String>>().into_iter().filter(|bag| { bag.to_string() != "no other".to_string()}).collect()
}

pub fn find_all_rules(lines: &Vec<String>) -> HashMap<String, HashMap<String, i32>> {
    let mut rules_hash: HashMap<String, HashMap<String, i32>> = HashMap::new();

    let bag_pattern = regex::Regex::new(r"(\w+) (\w+) bag").unwrap();
    let rule_pattern = regex::Regex::new(r"([0-9]) (\w+) (\w+)").unwrap();

    lines.iter().for_each(|line| {
        let first_bag_capture = &bag_pattern.captures_iter(line).collect::<Vec<regex::Captures>>()[0];
        let first_bag = first_bag_capture[1].to_string() + " " + &first_bag_capture[2].to_string();

        rules_hash.insert(first_bag.to_string(), HashMap::new());
        
        for rule_capture in rule_pattern.captures_iter(line) {
            let num_bag_type: i32 = rule_capture[1].parse().unwrap();
            let bag_type = rule_capture[2].to_string() + " " + &rule_capture[3].to_string();

            let rule_hash = rules_hash.get_mut(&first_bag).unwrap();
            rule_hash.insert(bag_type.to_string(), num_bag_type);
        }
    });

    rules_hash
}

pub fn get_rules_for_bag(bag: String, rules: &HashMap<String, HashMap<String, i32>>) -> HashMap<String, i32> {
    rules.get(&bag).unwrap().clone()
}

pub fn get_all_bags_bag_may_contain_recursive(bag: String, rules: &HashMap<String, HashMap<String, i32>>) -> Vec<String> {
    let bag_rules = get_rules_for_bag(bag, rules);
    let mut bag_list = bag_rules.keys().map(|key| key.to_string()).collect::<Vec<String>>();


    for idx in 0..bag_list.len() {
        bag_list.append(&mut get_all_bags_bag_may_contain_recursive(bag_list[idx].to_string(), &rules));
    }

    bag_list.into_iter().unique().collect()
}

pub fn count_bags_that_contain_bag(target_bag: String, bags: &Vec<String>, rules: &HashMap<String, HashMap<String, i32>>) -> i32 {
    bags.into_iter().fold(0, |bag_count, bag| bag_count + if get_all_bags_bag_may_contain_recursive(bag.to_string(), rules).contains(&target_bag) { 1 } else { 0 })
}

#[cfg(test)]
mod day7tests {
    use super::*;
    
    #[test]
    fn find_bag_types_works() {
        let test_data = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
            "bright white bags contain 1 shiny gold bag.".to_string(),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
            "faded blue bags contain no other bags.".to_string(),
            "dotted black bags contain no other bags.".to_string(),
        ];
        let bags = find_all_kinds_of_bag(&test_data);
        assert_eq!(bags.len(), 9);
        assert_eq!(bags[0], "light red");
        assert_eq!(bags[2], "muted yellow");
        assert_eq!(bags.contains(&"shiny gold".to_string()), true);
    }

    #[test]
    fn find_rules_works() {
        let test_data = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
            "bright white bags contain 1 shiny gold bag.".to_string(),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
            "faded blue bags contain no other bags.".to_string(),
            "dotted black bags contain no other bags.".to_string(),
        ];
        let rules = find_all_rules(&test_data);

        assert_eq!(rules.len(), 9);

        let light_red_bag_rules = get_rules_for_bag("light red".to_string(), &rules);

        assert_eq!(light_red_bag_rules.len(), 2);

        let faded_blue_bag_rules = get_rules_for_bag("faded blue".to_string(), &rules);

        assert_eq!(faded_blue_bag_rules.len(), 0);

        assert_eq!(light_red_bag_rules.contains_key(&"bright white".to_string()), true);
        assert_eq!(*light_red_bag_rules.get(&"bright white".to_string()).unwrap(), 1i32);
    }

    #[test]
    fn bag_recursion_works() {
        let test_data = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
            "bright white bags contain 1 shiny gold bag.".to_string(),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
            "faded blue bags contain no other bags.".to_string(),
            "dotted black bags contain no other bags.".to_string(),
        ];
        let rules = find_all_rules(&test_data);
        let bags_inside_light_red_bag = get_all_bags_bag_may_contain_recursive("light red".to_string(), &rules);

        assert_eq!(bags_inside_light_red_bag.len(), 7);

        let bags_that_contain_shiny_gold = count_bags_that_contain_bag("shiny gold".to_string(), &rules.keys().map(|key| key.to_string()).collect::<Vec<String>>(), &rules);

        assert_eq!(bags_that_contain_shiny_gold, 4);
    }
}