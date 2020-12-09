// Day 9: Encoding Error
// https://adventofcode.com/2020/day/9
use super::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use circular_queue::CircularQueue;
use itertools::Itertools;

pub fn display_day_menu(s: &mut Cursive) {
    let menu = SelectView::<i32>::new()
        .on_submit(menu_selection)
        .with_name("day_menu")
        .fixed_size((40, 15));

    s.add_layer(Dialog::around(menu).title("Day 9"));
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
            let bufreader = BufReader::new(File::open("inputs/day9.txt").unwrap());
            let inputs: Vec<i64> = bufreader.lines().map(|line| line.unwrap().parse().unwrap()).collect();

            Ok(format!("The first bad number in this stream is {}!", xmas_encoding_find_first_invalid_number(&inputs, 25)))
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
            let bufreader = BufReader::new(File::open("inputs/day9.txt").unwrap());
            let inputs: Vec<i64> = bufreader.lines().map(|line| line.unwrap().parse().unwrap()).collect();

            let first_invalid_number = xmas_encoding_find_first_invalid_number(&inputs, 25);
            let encryption_weakness = xmas_encoding_find_contiguous_set_sum_to(first_invalid_number, &inputs);
            Ok(format!("The encryption weakness with target value {} is: {:?}! 😎", first_invalid_number, encryption_weakness))
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

pub fn valid_next_number(next_num: i64, queue: &CircularQueue<i64>) -> bool {
    queue.iter().combinations(2).filter(|pair| (pair[0] + pair[1]) == next_num).count() > 0
}

pub fn xmas_encoding_find_first_invalid_number(inputs: &Vec<i64>, step_size: i64) -> i64 {
    let mut queue = CircularQueue::with_capacity(step_size as usize);

    // Preamble.
    for idx in 0 .. step_size {
        queue.push(inputs[idx as usize]);
    }

    // Find first invalid number.
    let mut first_invalid_number = 0;
    for idx in step_size .. inputs.len() as i64 {
        first_invalid_number = inputs[idx as usize];
        if !valid_next_number(first_invalid_number, &queue) { 
            break;
        }
        queue.push(first_invalid_number);
    }

    first_invalid_number
}

pub fn xmas_encoding_find_contiguous_set_sum_to(target: i64, inputs:&Vec<i64>) -> Result<i64, String> {
    let mut current_sum = 0;
    for set_start in 0 .. inputs.len() {
        current_sum = inputs[set_start];

        for set_end in set_start+1 .. inputs.len() {
            current_sum += inputs[set_end];

            if current_sum == target {
                // Found contiguous segment! Let's take a slice, and add the smallest & largest values together to get our result.
                let sliced_input = &inputs[set_start .. set_end+1];
                return Ok(sliced_input.iter().min().unwrap() + sliced_input.iter().max().unwrap());
            }
        }
    }

    Err(format!("Couldn't find it... {} -> {}", inputs.len(), current_sum))
}



#[cfg(test)]
mod day9tests {
    use super::*;
   
    #[test]
    fn valid_next_number_works() {
        let mut test_queue = CircularQueue::with_capacity(25);
        test_queue.push(20);
        for i in 1 .. 26 {
            if i != 20 {
                test_queue.push(i);
            }
        }

        assert_eq!(valid_next_number(26, &test_queue), true);
        assert_eq!(valid_next_number(49, &test_queue), true);
        assert_eq!(valid_next_number(100, &test_queue), false);
        assert_eq!(valid_next_number(50, &test_queue), false);

        test_queue.push(45);

        assert_eq!(valid_next_number(26, &test_queue), true);
        assert_eq!(valid_next_number(65, &test_queue), false);
        assert_eq!(valid_next_number(64, &test_queue), true);
        assert_eq!(valid_next_number(66, &test_queue), true);
    }

    #[test]
    fn find_first_invalid_number_works() {
        let test_inputs = vec![
            35,
            20,
            15,
            25,
            47,
            40,
            62,
            55,
            65,
            95,
            102,
            117,
            150,
            182,
            127,
            219,
            299,
            277,
            309,
            576,
        ];

        assert_eq!(xmas_encoding_find_first_invalid_number(&test_inputs, 5), 127);
    }

    #[test]
    fn contiguous_segment_find_works() {
        let test_inputs = vec![
            35,
            20,
            15,
            25,
            47,
            40,
            62,
            55,
            65,
            95,
            102,
            117,
            150,
            182,
            127,
            219,
            299,
            277,
            309,
            576,
        ];

        assert_eq!(xmas_encoding_find_contiguous_set_sum_to(127, &test_inputs), Ok(62));
        
    }
}