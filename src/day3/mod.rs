// Day 3: Toboggan Trajectory
// https://adventofcode.com/2020/day/3
use super::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn display_day_menu(s: &mut Cursive) {
    let menu = SelectView::<i32>::new()
        .on_submit(menu_selection)
        .with_name("day_menu")
        .fixed_size((40, 15));

    s.add_layer(Dialog::around(menu).title("Day 3"));
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
            let bufreader = BufReader::new(File::open("inputs/day3_1.txt").unwrap());
            let map_ylines: Vec<String> = bufreader
                .lines()
                .map(|line| {
                    return line.unwrap();
                })
                .collect();

            let tree_count = count_trees_in_map(&map_ylines, 3, 1, '#');

            Ok(format!("Trees thumped: {}", tree_count))
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
            let bufreader = BufReader::new(File::open("inputs/day3_1.txt").unwrap());
            let map_ylines: Vec<String> = bufreader
                .lines()
                .map(|line| {
                    return line.unwrap();
                })
                .collect();

            let slopes = vec![
                (1, 1),
                (3, 1),
                (5, 1),
                (7, 1),
                (1, 2),
            ];

            let tree_count = map_all_slopes_multiplied_together(&map_ylines, &slopes, '#');

            Ok(format!("🎄 x {}", tree_count))
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

pub fn count_trees_in_map(map_ylines: &Vec<String>, deltax: i32, deltay: i32, tree: char) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut tree_count: i32 = 0;
    
    while y + deltay < map_ylines.len() as i32 {
        y += deltay;
        x += deltax;
        
        if is_tree_at_location(&map_ylines[y as usize], x, tree) {
            tree_count += 1;
        }
    }
    tree_count
}

pub fn is_tree_at_location(yline: &String, x: i32, tree: char) -> bool {
    // Wrap X.
    let wrapped_x = x % yline.len() as i32;

    yline.chars().nth(wrapped_x as usize) == Some(tree)
}

pub fn map_all_slopes_multiplied_together(map_ylines: &Vec<String>, slopes: &Vec<(i32, i32)>, tree: char) -> i64 {
    slopes.iter().fold(1, |trees, slope| trees * count_trees_in_map(map_ylines, slope.0, slope.1, tree) as i64)
}

#[cfg(test)]
mod day3tests {
    use super::*;
    
    #[test]
    fn checking_for_tree_works() {
        let test_entries = vec![
            "..##.......".to_string()];

        
        assert_eq!(day3::is_tree_at_location(&test_entries[0], 0, '#'), false);
        assert_eq!(day3::is_tree_at_location(&test_entries[0], 2, '#'), true);
        assert_eq!(day3::is_tree_at_location(&test_entries[0], 2, 'X'), false);
    }

    #[test]
    fn x_wrap_works() {
        let test_entries = vec![
            "..##.......".to_string()];

        
        assert_eq!(day3::is_tree_at_location(&test_entries[0], 13, '#'), true);
        assert_eq!(day3::is_tree_at_location(&test_entries[0], 16, '#'), false);
    }

    #[test]
    fn map_tree_count_works() {
        let test_entries = vec![
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string()];
        
        assert_eq!(day3::count_trees_in_map(&test_entries, 3, 1, '#'), 7);
    }

    #[test]
    fn map_tree_count_all_slopes_works() {
        let test_entries = vec![
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string()];
        
        let test_slopes = vec![
            (1, 1),
            (3, 1),
            (5, 1),
            (7, 1),
            (1, 2),
        ];
        
        assert_eq!(day3::map_all_slopes_multiplied_together(&test_entries, &test_slopes, '#'), 336);
    }
}