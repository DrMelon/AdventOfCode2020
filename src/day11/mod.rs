// Day 11: Seating System
// https://adventofcode.com/2020/day/11
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

    s.add_layer(Dialog::around(menu).title("Day 10"));
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
            let bufreader = BufReader::new(File::open("inputs/day11.txt").unwrap());
            let inputs: Vec<String> = bufreader.lines().map(|line| line.unwrap()).collect();

            let grid = create_cellular_grid(&inputs);
            let complete_grid = run_grid_until_no_changes(&grid);

            Ok(format!("Filled seats: {}",count_total_occupied_seats(&complete_grid)))
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
            let bufreader = BufReader::new(File::open("inputs/day11.txt").unwrap());
            let inputs: Vec<String> = bufreader.lines().map(|line| line.unwrap()).collect();

            let grid = create_cellular_grid(&inputs);
            let complete_grid = run_grid_until_no_changes_star2(&grid);

            Ok(format!("Filled seats: {}",count_total_occupied_seats(&complete_grid)))
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

pub fn create_cellular_grid(inputs: &Vec<String>) -> (usize, usize, Vec<char>) {
    let width = inputs[0].len();
    let height = inputs.len();
    
    let mut grid_content = Vec::new();

    for line in inputs {
        for idx in 0 .. line.len() {
            grid_content.push(line.chars().nth(idx).unwrap());
        }
    }

    (width, height, grid_content)
}

pub fn get_char_in_grid(x: usize, y: usize, grid: &(usize, usize, Vec<char>)) -> Option<char> {
    if x < grid.0 && y < grid.1 {
        Some( grid.2[(y * grid.0) + x] )
    }
    else { None }
}

pub fn set_char_in_grid(x: usize, y: usize, grid: &mut (usize, usize, Vec<char>), set_char: char){
    if x < grid.0 && y < grid.1 {
        grid.2[(y * grid.0) + x] = set_char;
    }
}

pub fn get_num_occupied_seats_nearby(x: usize, y: usize, grid: &(usize, usize, Vec<char>)) -> i32 {
    let mut num_occupied = 0;
    for dx in ((x as i32 - 1) .. (x as i32 + 2)) {
        for dy in ((y as i32 - 1) .. (y as i32 + 2)) {
            if dx >= 0 && dy >= 0 {
                if !(dx == x as i32 && dy == y as i32){
                    num_occupied += if get_char_in_grid(dx as usize, dy as usize, grid) == Some('#') { 1 } else { 0 };
                }
            }
        }
    }

    num_occupied
}

pub fn get_num_occupied_seats_trace(x: usize, y: usize, grid: &(usize, usize, Vec<char>)) -> i32 {
    let mut num_occupied = 0;
    // scan directions
    for dx in -1 .. 2 {
        for dy in -1 .. 2 {
            if !(dx == 0 && dy == 0) {
                // step until we find a seat or we run off the edge
                let mut step_amt = 1;
                let mut step_x = x as i32 + (dx * step_amt);
                let mut step_y = y as i32 + (dy * step_amt);
                
                while(step_x >= 0 && step_x < grid.0 as i32 && step_y >= 0 && step_y < grid.1 as i32)
                {
                    
                    if get_char_in_grid(step_x as usize, step_y as usize, grid) == Some('#') {
                        num_occupied += 1;
                        break;
                    }

                    if get_char_in_grid(step_x as usize, step_y as usize, grid) == Some('L') {
                        break;
                    }

                    step_amt += 1;
                    step_x = x as i32 + (dx * step_amt);
                    step_y = y as i32 + (dy * step_amt);
                }
                
            }
        }
    }

    num_occupied
}

pub fn count_total_occupied_seats(grid: &(usize, usize, Vec<char>)) -> usize {
    grid.2.iter().filter(|chr| **chr == '#').count()
}

pub fn solve_grid_state_star1(grid:  &(usize, usize, Vec<char>)) -> (usize, usize, Vec<char>) {
    let mut new_state = grid.clone();

    for x in 0 .. grid.0 {
        for y in 0 .. grid.1 { 
            match get_char_in_grid(x, y, grid) {
                Some('L') => {
                    if get_num_occupied_seats_nearby(x, y, grid) == 0 {
                        set_char_in_grid(x, y, &mut new_state, '#');
                    }
                }
                Some('#') => {
                    if get_num_occupied_seats_nearby(x, y, grid) >= 4 {
                        set_char_in_grid(x, y, &mut new_state, 'L');
                    }
                }
                _ => {}
            }
        }
    }

    new_state
}

pub fn solve_grid_state_star2(grid:  &(usize, usize, Vec<char>)) -> (usize, usize, Vec<char>) {
    let mut new_state = grid.clone();

    for x in 0 .. grid.0 {
        for y in 0 .. grid.1 { 
            match get_char_in_grid(x, y, grid) {
                Some('L') => {
                    if get_num_occupied_seats_trace(x, y, grid) == 0 {
                        set_char_in_grid(x, y, &mut new_state, '#');
                    }
                }
                Some('#') => {
                    if get_num_occupied_seats_trace(x, y, grid) >= 5 {
                        set_char_in_grid(x, y, &mut new_state, 'L');
                    }
                }
                _ => {}
            }
        }
    }

    new_state
}

pub fn grids_equal(gridA:  &(usize, usize, Vec<char>), gridB: &(usize, usize, Vec<char>)) -> bool {
    let mut equal = true;

    equal &= gridA.0 == gridB.0;
    equal &= gridA.1 == gridB.1;
    
    let matchcount = gridA.2.iter().zip(gridB.2.iter()).filter(|&(a, b)| a == b).count();

    equal &= gridA.2.len() == matchcount && gridB.2.len() == matchcount;

    equal
}

pub fn run_grid_until_no_changes(grid:  &(usize, usize, Vec<char>)) -> (usize, usize, Vec<char>) {
    let mut stepped_grid = grid.clone();

    let mut iter = 0;
    while !grids_equal(&stepped_grid, &solve_grid_state_star1(&stepped_grid)) {
        iter += 1;
        if iter < 200 && iter % 2 == 0 {
            save_grid_to_image(&stepped_grid, format!("out/{}.png", iter));
        }
        stepped_grid = solve_grid_state_star1(&stepped_grid);
    }

    stepped_grid
}

pub fn run_grid_until_no_changes_star2(grid:  &(usize, usize, Vec<char>)) -> (usize, usize, Vec<char>) {
    let mut stepped_grid = grid.clone();

    let mut iter = 0;
    while !grids_equal(&stepped_grid, &solve_grid_state_star2(&stepped_grid)) {
        iter += 1;
        if iter < 200 && iter % 2 == 0 {
            save_grid_to_image(&stepped_grid, format!("out/z{}.png", iter));
        }
        stepped_grid = solve_grid_state_star2(&stepped_grid);
    }

    stepped_grid
}

pub fn save_grid_to_image(grid: &(usize, usize, Vec<char>), filename: String) {
    let mut img = RgbImage::new(grid.0 as u32, grid.1 as u32);
    for y in 0 .. grid.1 {
        for x in 0 .. grid.0 {
            match get_char_in_grid(x, y, grid) {
                Some('L') => {
                    img.put_pixel(x as u32, y as u32, Rgb([96, 96, 96]));
                }
                Some('#') => {
                    img.put_pixel(x as u32, y as u32, Rgb([127, 148, 127]));
                }
                Some('.') => {
                    img.put_pixel(x as u32, y as u32, Rgb([48, 48, 48]));
                }
                _=>{}
            }
        }
    } 

    img.save(filename).unwrap();
}

#[cfg(test)]
mod day11tests {
    use super::*;

    fn get_test_data_empty() -> Vec<String> {
        vec![
            "L.LL.LL.LL".to_string(),
            "LLLLLLL.LL".to_string(),
            "L.L.L..L..".to_string(),
            "LLLL.LL.LL".to_string(),
            "L.LL.LL.LL".to_string(),
            "L.LLLLL.LL".to_string(),
            "..L.L.....".to_string(),
            "LLLLLLLLLL".to_string(),
            "L.LLLLLL.L".to_string(),
            "L.LLLLL.LL".to_string(),
        ]
    }

    fn get_test_data_full() -> Vec<String> {
        vec![
            "#.##.##.##".to_string(),
            "#######.##".to_string(),
            "#.#.#..#..".to_string(),
            "####.##.##".to_string(),
            "#.##.##.##".to_string(),
            "#.#####.##".to_string(),
            "..#.#.....".to_string(),
            "##########".to_string(),
            "#.######.#".to_string(),
            "#.#####.##".to_string(),
        ]
    }

    fn get_test_data_final() -> Vec<String> {
        vec![
            "#.#L.L#.##".to_string(),
            "#LLL#LL.L#".to_string(),
            "L.#.L..#..".to_string(),
            "#L##.##.L#".to_string(),
            "#.#L.LL.LL".to_string(),
            "#.#L#L#.##".to_string(),
            "..L.L.....".to_string(),
            "#L#L##L#L#".to_string(),
            "#.LLLLLL.L".to_string(),
            "#.#L#L#.##".to_string(),
        ]
    }

    fn get_test_data_final_star2() -> Vec<String> {
        vec![
            "#.L#.L#.L#".to_string(),
            "#LLLLLL.LL".to_string(),
            "L.L.L..#..".to_string(),
            "##L#.#L.L#".to_string(),
            "L.L#.LL.L#".to_string(),
            "#.LLLL#.LL".to_string(),
            "..#.L.....".to_string(),
            "LLL###LLL#".to_string(),
            "#.LLLLL#.L".to_string(),
            "#.L#LL#.L#".to_string(),
        ]
    }

    fn get_test_data_trace() -> Vec<String> {
        vec![
            "...#...#.".to_string(),
            "...#.....".to_string(),
            ".#.......".to_string(),
            ".........".to_string(),
            "..#L....#".to_string(),
            "....#....".to_string(),
            ".........".to_string(),
            "#........".to_string(),
            "...#.....".to_string()
        ]       
    }

    fn get_test_data_trace_none() -> Vec<String> {
        vec![
            ".##.##.".to_string(),
            "#.#.#.#".to_string(),
            "##...##".to_string(),
            "...L...".to_string(),
            "##...##".to_string(),
            "#.#.#.#".to_string(),
            ".##.##.".to_string(),
        ]       
    }

    #[test]
    fn test_grid_creation_empty() {
        let grid = create_cellular_grid(&get_test_data_empty());
        assert_eq!(grid.0, 10);
        assert_eq!(grid.1, 10);
        assert_eq!(grid.2.len(), 100);
        assert_eq!(get_char_in_grid(0, 0, &grid), Some('L'));
    }

    fn get_test_data_grid_empty() -> (usize, usize, Vec<char>) {
        create_cellular_grid(&get_test_data_empty())
    }

    fn get_test_data_grid_full() -> (usize, usize, Vec<char>) {
        create_cellular_grid(&get_test_data_full())
    }

    fn get_test_data_grid_final() -> (usize, usize, Vec<char>) {
        create_cellular_grid(&get_test_data_final())
    }

    fn get_test_data_grid_final_star2() -> (usize, usize, Vec<char>) {
        create_cellular_grid(&get_test_data_final_star2())
    }
    
    #[test]
    fn grid_step_works() {
        let test_grid = get_test_data_grid_empty();
        let test_grid_full = get_test_data_grid_full();

        let stepped_grid = solve_grid_state_star1(&test_grid);
        assert_eq!(get_char_in_grid(0, 0, &stepped_grid), get_char_in_grid(0, 0, &test_grid_full));
        assert_eq!(grids_equal(&stepped_grid, &test_grid_full), true);

        assert_eq!(get_num_occupied_seats_nearby(2, 2, &stepped_grid), 6);
    }

    #[test]
    fn grid_iterated_count_works() {
        let test_grid = get_test_data_grid_empty();
        
        let stepped_grid = run_grid_until_no_changes(&test_grid);
        assert_eq!(count_total_occupied_seats(&stepped_grid), 37);
        assert_eq!(grids_equal(&stepped_grid, &get_test_data_grid_final()), true);

        let stepped_grid = run_grid_until_no_changes_star2(&test_grid);
        assert_eq!(count_total_occupied_seats(&stepped_grid), 26);
        assert_eq!(grids_equal(&stepped_grid, &get_test_data_grid_final_star2()), true);
    }
    
    #[test]
    fn seats_trace_works() {
        let test_grid = create_cellular_grid(&get_test_data_trace());

        assert_eq!(get_num_occupied_seats_trace(3, 4, &test_grid), 8);

        let test_grid = create_cellular_grid(&get_test_data_trace_none());

        assert_eq!(get_num_occupied_seats_trace(3, 3, &test_grid), 0);
    }
}