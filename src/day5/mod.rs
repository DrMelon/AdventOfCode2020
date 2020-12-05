// Day 5: Binary Boarding
// https://adventofcode.com/2020/day/5
use super::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn display_day_menu(s: &mut Cursive) {
    let menu = SelectView::<i32>::new()
        .on_submit(menu_selection)
        .with_name("day_menu")
        .fixed_size((40, 15));

    s.add_layer(Dialog::around(menu).title("Day 5"));
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
            let bufreader = BufReader::new(File::open("inputs/day5_1.txt").unwrap());
            let lines : Vec<String> = bufreader.lines().map(|line| line.unwrap()).collect();

            let max_seat_id = lines.iter().fold(0, |highest_id, line| highest_id.max(seat_id_from_seat_location(process_boarding_token(line))));
            
            Ok(format!("Highest Seat ID: {}", max_seat_id))
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
            let bufreader = BufReader::new(File::open("inputs/day5_1.txt").unwrap());
            
            
            Ok(format!("Yump!"))
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

pub fn process_boarding_token(token: &String) -> (i32, i32) {
    // Token Format: 
    // A 10 character string.
    // First 7 chars are F or B
    // Last 3 chars are L or R
    // A BSP tree, essentially.

    let mut maxrow = 127;
    let mut minrow = 0;
    let mut maxcol = 7;
    let mut mincol = 0;

    for idx in 0..10 {
        match token.chars().nth(idx).unwrap() {
            'F' => {
                maxrow -= ((maxrow - minrow) / 2) + 1;
            }
            'B' => {
                minrow += ((maxrow - minrow) / 2) + 1;
            },
            'L' => {
                maxcol -= ((maxcol - mincol) / 2) + 1;
            },
            'R' => {
                mincol += ((maxcol - mincol) / 2) + 1;
            }
            _ => {}
        }
    }

    (maxrow, maxcol)
}

pub fn seat_id_from_seat_location(seat: (i32, i32)) -> i32 {
    (seat.0 * 8) + seat.1
}

#[cfg(test)]
mod day5tests {
    use super::*;
    
    #[test]
    fn process_token_works() {
        let test_tokens = vec![
            "FBFBBFFRLR".to_string(),
            "BFFFBBFRRR".to_string(),
            "FFFBBBFRRR".to_string(),
            "BBFFBBFRLL".to_string(),
        ];

        assert_eq!(process_boarding_token(&test_tokens[0]), (44, 5));
        assert_eq!(process_boarding_token(&test_tokens[1]), (70, 7));
        assert_eq!(process_boarding_token(&test_tokens[2]), (14, 7));
        assert_eq!(process_boarding_token(&test_tokens[3]), (102, 4));
    }

    #[test]
    fn seat_id_calc_works() {
        let test_seats = vec![
            (44, 5),
            (70, 7),
            (14, 7),
            (102, 4),
        ];

        assert_eq!(seat_id_from_seat_location(test_seats[0]), 357);
        assert_eq!(seat_id_from_seat_location(test_seats[1]), 567);
        assert_eq!(seat_id_from_seat_location(test_seats[2]), 119);
        assert_eq!(seat_id_from_seat_location(test_seats[3]), 820);
    }
}