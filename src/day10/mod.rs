// Day 10: Adapter Array
// https://adventofcode.com/2020/day/10
use super::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

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
            let bufreader = BufReader::new(File::open("inputs/day10.txt").unwrap());
            let inputs: Vec<i32> = bufreader.lines().map(|line| line.unwrap().parse().unwrap()).collect();

            let device_joltage = get_device_port_rating(&inputs);
            let adapter_chain = get_adapter_chain(0, device_joltage, &inputs);
            let one_jumps = count_jumps_of_length(&adapter_chain, 1);
            let three_jumps = count_jumps_of_length(&adapter_chain, 3);
            let combined = one_jumps * three_jumps;

            Ok(format!("Number of adapters: {}\n1-jolt jumps:{}\n3-jolt jumps:{}\nFinal:{}", adapter_chain.len(), one_jumps, three_jumps, combined))
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
            let bufreader = BufReader::new(File::open("inputs/day10.txt").unwrap());
            let inputs: Vec<i32> = bufreader.lines().map(|line| line.unwrap().parse().unwrap()).collect();

            Ok(format!("The total number of configurations is: {}", get_total_number_of_chains(&inputs)))
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

pub fn get_joltage_adapter_delta(adapter_a: i32, adapter_b: i32) -> i32 {
    adapter_b - adapter_a
}

pub fn can_joltage_adapters_connect(lower: i32, higher: i32) -> bool {
    let delta = get_joltage_adapter_delta(lower, higher);
    delta <= 3 && delta > 0
}

pub fn get_device_port_rating(adapters: &Vec<i32>) -> i32 {
    adapters.iter().max().unwrap() + 3
}

pub fn get_viable_adapters_for_joltage(source_joltage: i32, adapters: &Vec<i32>) -> Vec<i32> {
    adapters.into_iter().filter(|adapter| can_joltage_adapters_connect(source_joltage, **adapter)).map(|a| *a).sorted().collect::<Vec<i32>>()
}
pub fn get_adapter_chain(start_joltage: i32, target_joltage: i32, adapters: &Vec<i32>) -> Vec<i32> {
    let mut connected_adapters = Vec::new();

    connected_adapters.push(start_joltage);

    while !can_joltage_adapters_connect(*connected_adapters.last().unwrap(), target_joltage) {
        connected_adapters.push(*get_viable_adapters_for_joltage(*connected_adapters.last().unwrap(), adapters).first().unwrap());
    }

    connected_adapters.push(target_joltage);

    connected_adapters
}

pub fn get_max_jump_adapter_chain(start_joltage: i32, target_joltage: i32, adapters: &Vec<i32>) -> Vec<i32> {
    let mut connected_adapters = Vec::new();

    connected_adapters.push(start_joltage);

    while !can_joltage_adapters_connect(*connected_adapters.last().unwrap(), target_joltage) {
        connected_adapters.push(*get_viable_adapters_for_joltage(*connected_adapters.last().unwrap(), adapters).last().unwrap());
    }

    connected_adapters.push(target_joltage);

    connected_adapters  
}


pub fn count_jumps_of_length(adapter_chain: &Vec<i32>, jump_length: i32) -> i32 {
    let mut last_adapter = adapter_chain[0];
    adapter_chain.iter().filter(|adapter| { let fits = get_joltage_adapter_delta(last_adapter, **adapter) == jump_length; last_adapter = **adapter; fits}).count() as i32
}

pub fn is_valid_chain(start_joltage:i32, target_joltage: i32, adapter_chain: &Vec<i32>) -> bool {
    let mut last_adapter = adapter_chain[0];
    start_joltage == adapter_chain[0] && 
    target_joltage == *adapter_chain.last().unwrap() &&
    adapter_chain.iter().skip(1).filter(|adapter| { let fits = can_joltage_adapters_connect(last_adapter, **adapter); last_adapter = **adapter; fits}).count() == adapter_chain.len() - 1
}

pub fn get_total_number_of_chains(adapters: &Vec<i32>) -> i64 {
    // first, sort adapter numbers descending
    let mut sorted_adapters = adapters.into_iter().sorted().map(|a| *a).collect::<Vec<i32>>();
    sorted_adapters.reverse();
    sorted_adapters.push(0);

    let mut known_chains = vec![1 as i64];

    sorted_adapters.iter().enumerate().skip(1).for_each(|(idx, &_adapter)| {
        let mut jumpback_matches: i64 = 0;

        let look_behind_index = idx as i32 - 1;
        if look_behind_index >= 0 {
            jumpback_matches += if can_joltage_adapters_connect(sorted_adapters[idx], sorted_adapters[look_behind_index as usize]) { known_chains[look_behind_index as usize] } else { 0 };
        }
        let look_behind_index = idx as i32 - 2;
        if look_behind_index >= 0 {
            jumpback_matches += if can_joltage_adapters_connect(sorted_adapters[idx], sorted_adapters[look_behind_index as usize]) { known_chains[look_behind_index as usize] } else { 0 };
        }
        let look_behind_index = idx as i32 - 3;
        if look_behind_index >= 0 {
            jumpback_matches += if can_joltage_adapters_connect(sorted_adapters[idx], sorted_adapters[look_behind_index as usize]) { known_chains[look_behind_index as usize] } else { 0 };
        }        
        known_chains.push(jumpback_matches);
        
    });

    *known_chains.last().unwrap() as i64
}

#[cfg(test)]
mod day10tests {
    use super::*;
   
    fn get_test_data_small() -> Vec<i32> {
        vec![
            16,
            10,
            15,
            5,
            1,
            11,
            7,
            19,
            6,
            12,
            4
        ]
    }

    fn get_test_data_large() -> Vec<i32> {
        vec![
            28,
            33,
            18,
            42,
            31,
            14,
            46,
            20,
            48,
            47,
            24,
            23,
            49,
            45,
            19,
            38,
            39,
            11,
            1,
            32,
            25,
            35,
            8,
            17,
            7,
            9,
            4,
            2,
            34,
            10,
            3
        ]
    }

    #[test]
    fn device_port_rating_works() {
        let test_data = get_test_data_small();

        assert_eq!(get_device_port_rating(&test_data), 22);
    }

    #[test]
    fn joltage_connectivity_works() {
        let test_joltage = 10;

        assert_eq!(can_joltage_adapters_connect(test_joltage, 11), true);
        assert_eq!(can_joltage_adapters_connect(test_joltage, 12), true);
        assert_eq!(can_joltage_adapters_connect(test_joltage, 13), true);
        assert_eq!(can_joltage_adapters_connect(test_joltage, 14), false);
        assert_eq!(can_joltage_adapters_connect(test_joltage, 9), false);
    }

    #[test]
    fn viable_adapter_check_works() {
        let test_data = get_test_data_small();
        let viable_adapters = get_viable_adapters_for_joltage(0, &test_data);
        assert_eq!(viable_adapters.len(), 1);

        let viable_adapters = get_viable_adapters_for_joltage(*viable_adapters.last().unwrap(), &test_data);
        assert_eq!(viable_adapters.len(), 1);

        let viable_adapters = get_viable_adapters_for_joltage(*viable_adapters.last().unwrap(), &test_data);
        assert_eq!(viable_adapters.len(), 3);
    }

    #[test]
    fn adapter_chain_works() {
        let test_data = get_test_data_small();
        let device_joltage = get_device_port_rating(&test_data);
        let adapter_chain = get_adapter_chain(0, device_joltage, &test_data);

        assert_eq!(adapter_chain.len(), 13);
        assert_eq!(count_jumps_of_length(&adapter_chain, 1), 7);
        assert_eq!(count_jumps_of_length(&adapter_chain, 3), 5);

        let test_data_large = get_test_data_large();
        let device_joltage = get_device_port_rating(&test_data_large);
        let adapter_chain = get_adapter_chain(0, device_joltage, &test_data_large);

        assert_eq!(adapter_chain.len(), 33);
        assert_eq!(count_jumps_of_length(&adapter_chain, 1), 22);
        assert_eq!(count_jumps_of_length(&adapter_chain, 3), 10);
    }

    #[test]
    fn max_jump_adapter_chain_works() {
        let test_data = get_test_data_small();
        let device_joltage = get_device_port_rating(&test_data);
        let adapter_chain = get_max_jump_adapter_chain(0, device_joltage, &test_data);

        assert_eq!(adapter_chain.len() - 2, 8);
    }

    #[test]
    fn chain_validator_works() {
        let test_data = get_test_data_small();
        let device_joltage = get_device_port_rating(&test_data);
        let adapter_chain = get_adapter_chain(0, device_joltage, &test_data);
        
        assert_eq!(is_valid_chain(0, 22, &adapter_chain), true);
        assert_eq!(is_valid_chain(3, 14, &adapter_chain), false);
        assert_eq!(is_valid_chain(0, 22, &test_data), false);
    }

    #[test]
    fn count_chains_works() {
        let test_data = get_test_data_small();
        let test_data_large = get_test_data_large();

        assert_eq!(get_total_number_of_chains(&test_data), 8);
        assert_eq!(get_total_number_of_chains(&test_data_large), 19208);
    }
}