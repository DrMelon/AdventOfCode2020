// Day 1: Report Repair
// https://adventofcode.com/2020/day/1
use super::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn display_day_menu(s: &mut Cursive) {
    let menu = SelectView::<i32>::new()
    .on_submit(menu_selection)
    .with_name("day_menu")
    .fixed_size((40,15));

    s.add_layer(Dialog::around(menu).title("Day 1"));
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
        },
        1 => {
            second_star(s);
        },
        999 => {
            s.pop_layer();
        },
        _ => {}
    }
}

// Iterate pairs of numbers to find a pair that sums to 2020,
// and then multiply the pair to find the answer for the first star.
pub fn first_star(s: &mut Cursive) {
    // Create async dialog for this.
    let async_view = AsyncView::new_with_bg_creator(s, move || {
        // Load input file and parse it into a vec of ints
        let bufreader = BufReader::new(File::open("inputs/day1_1.txt").unwrap());
        let numbers: Vec<i32> = bufreader.lines().map(|line| {return line.unwrap().parse().unwrap();}).collect();

        // Process pairs of numbers.
        let mut results = (0, 0);
        'outerloop: for idx in 0 .. numbers.len() {
            let a = &numbers[idx];
            for b in &numbers[idx..] {
                if a + b == 2020 
                {
                    results = (*a, *b);
                    break 'outerloop;
                }
            }
        }

        Ok(format!("Done! {} * {} = {}", results.0, results.1, results.0 * results.1))
    }, TextView::new).with_height(15).with_width(30);
   
    s.add_layer(Dialog::around(async_view).title("1st Star ⭐").button("Neat!", |s| {s.pop_layer();}));
}

pub fn second_star(s: &mut Cursive) {
    // Create async dialog for this.
    let async_view = AsyncView::new_with_bg_creator(s, move || {
       // Load input file and parse it into a vec of ints
       let bufreader = BufReader::new(File::open("inputs/day1_2.txt").unwrap());
       let numbers: Vec<i32> = bufreader.lines().map(|line| {return line.unwrap().parse().unwrap();}).collect();

       // Process triplets of numbers.
       let mut results = (0, 0, 0);
       'outerloop: for idx in 0 .. numbers.len() {
           let a = &numbers[idx];
           let bslice = &numbers[idx..];
           for bidx in 0 .. bslice.len() {
               let b = &bslice[bidx];
               for c in &bslice[bidx..] {
                   if a + b + c == 2020 
                       {
                           results = (*a, *b, *c);
                           break 'outerloop;
                       }
               }
               
           }
       }

       Ok(format!("Done! {} * {} * {} = {}", results.0, results.1, results.2, results.0 * results.1 * results.2))
   }, TextView::new).with_height(15).with_width(30);
  
   s.add_layer(Dialog::around(async_view).title("2nd Star ⭐").button("Ah, cool!", |s| {s.pop_layer();}));
}
