// Day 12: Rain Risk
// https://adventofcode.com/2020/day/12
use super::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;
use image::*;

pub fn display_day_menu(s: &mut Cursive) {
    let menu = SelectView::<i32>::new()
        .on_submit(menu_selection)
        .with_name("day_menu")
        .fixed_size((40, 15));

    s.add_layer(Dialog::around(menu).title("Day 12"));
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
            let bufreader = BufReader::new(File::open("inputs/day12.txt").unwrap());
            let inputs: Vec<String> = bufreader.lines().map(|line| line.unwrap()).collect();

            let ship_state = run_instruction_set_on_ship(&inputs, &ShipState::new());

            render_ship_path(&inputs, (ship_state.max_x - ship_state.min_x).abs(), (ship_state.max_y - ship_state.min_y).abs());

            Ok(format!("Final Ship State: X: {} Y: {} H: {}, \nManhattan Distance: {}", ship_state.x, ship_state.y, ship_state.h, get_manhattan_distance_ship(&ship_state)))
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
            let bufreader = BufReader::new(File::open("inputs/day12.txt").unwrap());
            let inputs: Vec<String> = bufreader.lines().map(|line| line.unwrap()).collect();

            Ok(format!(""))
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

pub fn process_line_into_instruction(line: &String) -> (char, i32) {
    (line.chars().next().unwrap(), line[1..].parse().unwrap())
}

#[derive(Clone)]
pub struct ShipState {
    x: i32,
    y: i32,
    h: i32,
    max_x: i32,
    min_x: i32,
    max_y: i32,
    min_y: i32
}

impl ShipState {
    pub fn new() -> ShipState {
        ShipState {
            x: 0,
            y: 0,
            h: 0,
            max_x: 0,
            min_x: 0,
            max_y: 0,
            min_y: 0
        }
    }
}


pub fn run_instruction_on_ship(instruction: (char, i32), ship_state: &ShipState) -> ShipState {
    let mut new_state = ship_state.clone();

    match instruction.0 {
        'N' => {
            new_state.y -= instruction.1;
        },
        'S' => {
            new_state.y += instruction.1;
        }
        'W' => {
            new_state.x -= instruction.1;
        }
        'E' => {
            new_state.x += instruction.1;
        }
        'L' => {
            new_state.h += instruction.1;
        }
        'R' => {
            new_state.h -= instruction.1;
        }
        'F' => {
            let heading_x = f32::cos((ship_state.h as f32).to_radians());
            let heading_y = -f32::sin((ship_state.h as f32).to_radians());

            new_state.x += heading_x as i32 * instruction.1;
            new_state.y += heading_y as i32 * instruction.1;

            
        }
        _ => {}
    }

    if(new_state.x < new_state.min_x) {
        new_state.min_x = new_state.x;
    }
    if(new_state.y < new_state.min_y) {
        new_state.min_y = new_state.y;
    }
    if(new_state.x > new_state.max_x) {
        new_state.max_x = new_state.x;
    }
    if(new_state.y > new_state.max_y) {
        new_state.max_y = new_state.y;
    }

    new_state
}

pub fn run_instruction_set_on_ship(instructions: &Vec<String>, ship_state: &ShipState) -> ShipState {
    let mut new_state = ship_state.clone();
    for inst in instructions {
        new_state = run_instruction_on_ship(process_line_into_instruction(inst), &new_state);
    }

    new_state
}

pub fn render_ship_path(instructions: &Vec<String>, width: i32, height: i32) {
    let processed_instructions_list: Vec<(char, i32)> = instructions.into_iter().map(|line| process_line_into_instruction(line)).collect();
    
    let mut ship_state = ShipState::new();

    let mut img = RgbImage::new(((width)*2) as u32, ((height)*2) as u32);

    let mut idx = 0;
    for inst in processed_instructions_list {
        idx += 1;
        
        let ship_state_old = ship_state.clone();

        ship_state = run_instruction_on_ship(inst, &ship_state);

        let dx = ship_state.x - ship_state_old.x;
        let dy = ship_state.y - ship_state_old.y;
        
        img.put_pixel((ship_state.x + img.width() as i32 /2) as u32, (ship_state.y + img.width() as i32 /2) as u32, Rgb([255, 0, 0]));

        if dx < 0 {
            for x in ship_state.x .. ship_state_old.x {
                img.put_pixel((x + img.width() as i32 /2) as u32, (ship_state_old.y + img.width() as i32 /2) as u32, Rgb([255, 0, 0]));
            }
        } else if dx > 0 {
            for x in ship_state_old.x .. ship_state.x {
                img.put_pixel((x + img.width() as i32 /2) as u32, (ship_state_old.y + img.width() as i32 /2) as u32, Rgb([255, 0, 0]));
            }
        }
        if dy < 0 {
            for y in ship_state.y .. ship_state_old.y {
                img.put_pixel((ship_state_old.x + img.width() as i32 /2) as u32, (y + img.width() as i32 /2) as u32, Rgb([255, 0, 0]));
            }
        } else if dy > 0 {
            for y in ship_state_old.y .. ship_state.y {
                img.put_pixel((ship_state_old.x + img.width() as i32 /2) as u32, (y + img.width() as i32 /2) as u32, Rgb([255, 0, 0]));
            }
        }

        img.save(format!("out/ship{}.png", idx));
    }
        
} 

pub fn get_manhattan_distance_ship(ship_state: &ShipState) -> i32 {
    ship_state.x.abs() + ship_state.y.abs()
}


#[cfg(test)]
mod day12tests {
    use super::*;

    fn get_test_program() -> Vec<String> {
        vec![
        "F10".to_string(),
        "N3".to_string(),
        "F7".to_string(),
        "R90".to_string(),
        "F11".to_string(),
        ]
    }

    #[test]
    fn command_processing_works() {
        let test_data = get_test_program();
        let test_line = test_data[0].to_string();
        let instruction = process_line_into_instruction(&test_line);

        assert_eq!(instruction.0, 'F');
        assert_eq!(instruction.1, 10);
    }

    #[test]
    fn command_step_works() {
        let test_data = get_test_program();
        let mut ship_state = ShipState::new();

        ship_state = run_instruction_set_on_ship(&test_data, &ship_state);
        render_ship_path(&test_data, ship_state.max_x - ship_state.min_x, ship_state.max_y - ship_state.min_y);

        assert_eq!(ship_state.x, 17);
        assert_eq!(ship_state.y, 8);
        assert_eq!(ship_state.h, -90);
        assert_eq!(get_manhattan_distance_ship(&ship_state), 25);
    }

}