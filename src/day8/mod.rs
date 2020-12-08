// Day 8: Handheld Halting
// https://adventofcode.com/2020/day/8
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

    s.add_layer(Dialog::around(menu).title("Day 8"));
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
            let bufreader = BufReader::new(File::open("inputs/day8.txt").unwrap());
            let program: Vec<String> = bufreader.lines().map(|line| line.unwrap()).collect();

            let recursion_state = run_program_until_terminated(&ProgramState::new(), &program);
            let last_instruction = recursion_state.last_instruction.unwrap();
            Ok(format!("Okay, so:\n Acc: {}\n Pc: {}\n\n Li: {}:{} {}\n Tr: {}", recursion_state.accumulator, recursion_state.program_counter, last_instruction.0, last_instruction.1, last_instruction.2, recursion_state.term_reason))
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
            let bufreader = BufReader::new(File::open("inputs/day8.txt").unwrap());
            let program: Vec<String> = bufreader.lines().map(|line| line.unwrap()).collect();

            let (fixed_program, fixed_at) = program_autofix(&program);
            let fixed_state = run_program_until_terminated(&ProgramState::new(), &fixed_program);
            let last_instruction = fixed_state.last_instruction.unwrap();
            Ok(format!("Okay, so fixed program at line {}:\n Acc: {}\n Pc: {}\n\n Li: {}:{} {}\n Tr: {}", fixed_at+1, fixed_state.accumulator, fixed_state.program_counter, last_instruction.0, last_instruction.1, last_instruction.2, fixed_state.term_reason))

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

#[derive(Clone)]
pub struct ProgramState {
    accumulator: i32,
    program_counter: i32,
    last_instruction: Option<(i32, String, i32)>,
    visited_indices: Vec<i32>,
    terminated: bool,
    term_reason: i32
}

impl ProgramState {
    pub fn new() -> ProgramState {
        ProgramState {
            accumulator: 0,
            program_counter: 0,
            last_instruction: None,
            visited_indices: Vec::new(),
            terminated: false,
            term_reason: 0,
        }
    }
}

pub fn get_instruction(line: &String) -> (String, i32) {
    let split_line = line.split(' ').map(|s| s.to_string()).collect::<Vec<String>>();
    (split_line[0].to_string(), split_line[1].parse().unwrap())
}

pub fn step_program_forward(current_state: &ProgramState, program: &Vec<String>) -> ProgramState {
    if current_state.terminated {
        return current_state.clone();
    }

    let mut new_state = current_state.clone();

    if program.len() == current_state.program_counter as usize {
        new_state.terminated = true;
        new_state.term_reason = 0;
        return new_state;
    }

    let current_line = &program[current_state.program_counter as usize];
    
    new_state.visited_indices.push(current_state.program_counter);

    let (operator, operand) = get_instruction(&current_line);

    match operator.as_str() {
        "acc" => {
            new_state.accumulator += operand;
            new_state.program_counter += 1;
        },
        "jmp" => {
            new_state.program_counter += operand;
        }
        "nop" => {
            new_state.program_counter += 1;
        }
        _ => {}
    }

    new_state.last_instruction = Some((current_state.program_counter, operator, operand));

    // Check that we didn't visit the same instruction twice.
    if new_state.visited_indices.len() != new_state.visited_indices.clone().into_iter().unique().collect::<Vec<i32>>().len() {
        new_state = current_state.clone();
        new_state.terminated = true;
        new_state.term_reason = 1;
    }

    

    new_state
}

pub fn run_program_x_steps(current_state: &ProgramState, program: &Vec<String>, steps: i32) -> ProgramState {
    let mut new_state = current_state.clone();

    for _ in 0 .. steps {
        new_state = step_program_forward(&new_state, program);
    }


    new_state
}

pub fn run_program_until_terminated(current_state: &ProgramState, program: &Vec<String>) -> ProgramState {
    let mut new_state = current_state.clone();

    while !new_state.terminated {
        new_state = step_program_forward(&new_state, program);
    }

    new_state
}

pub fn program_autofix(broken_program: &Vec<String>) -> (Vec<String>, i32) {
    let mut new_program = broken_program.clone();
    let mut line_fix = 0;
    let mut program_state = run_program_until_terminated(&ProgramState::new(),&new_program);

    while program_state.term_reason == 1 {

        // Get visited indices from last run.
        let visited_indices = program_state.visited_indices.clone();

        for idx in visited_indices {
            // Manipulate the program. 
            let mut temp_new_program = new_program.clone();
            if get_instruction(&temp_new_program[idx as usize]).0.as_str() == "jmp" {
                temp_new_program[idx as usize] = "nop +0".to_string();
            }
            else if get_instruction(&temp_new_program[idx as usize]).0.as_str() == "nop" {
                temp_new_program[idx as usize] = format!("jmp {}", get_instruction(&temp_new_program[idx as usize]).1); 
            }

            let test_fix_program = run_program_until_terminated(&ProgramState::new(),&temp_new_program);
            if test_fix_program.term_reason == 0 {
                line_fix = idx;
                new_program = temp_new_program.clone();
                break;
            }
        }

        program_state = run_program_until_terminated(&ProgramState::new(),&new_program);
    }

    (new_program, line_fix)
}


#[cfg(test)]
mod day8tests {
    use super::*;
    
    #[test]
    pub fn test_operators() {
        let test_program = vec![
            "acc +1".to_string(),
            "nop +0".to_string(),
            "jmp -2".to_string(),
        ];

        let initial_program_state = ProgramState::new();

        let run_acc = step_program_forward(&initial_program_state, &test_program);
        let run_nop = step_program_forward(&run_acc, &test_program);
        let run_jmp = step_program_forward(&run_nop, &test_program);

        assert_eq!(run_acc.accumulator, 1);
        assert_eq!(run_nop.program_counter, 2);
        assert_eq!(run_jmp.program_counter, 0);
    }

    #[test]
    pub fn test_run_x_steps() {
        let test_program = vec![
            "acc +1".to_string(),
            "nop +0".to_string(),
            "jmp -2".to_string(),
        ];

        let initial_program_state = ProgramState::new();

        let run_state = run_program_x_steps(&initial_program_state, &test_program, 2);

        assert_eq!(run_state.accumulator, 1);
        assert_eq!(run_state.program_counter, 2);
        assert_eq!(run_state.terminated, false);
    }

    #[test]
    pub fn test_until_normal_termination() {
        let test_program = vec![
            "acc +1".to_string(),
            "nop +0".to_string(),
            "nop +0".to_string(),
        ];

        let initial_program_state = ProgramState::new();

        let run_state = run_program_until_terminated(&initial_program_state, &test_program);

        assert_eq!(run_state.accumulator, 1);
        assert_eq!(run_state.program_counter, 3);
        assert_eq!(run_state.term_reason, 0);
    }

    #[test]
    pub fn test_until_repeated_instruction() {
        let test_program = vec![
            "acc +1".to_string(),
            "nop +0".to_string(),
            "jmp -2".to_string(),
        ];

        let initial_program_state = ProgramState::new();

        let run_state = run_program_until_terminated(&initial_program_state, &test_program);

        assert_eq!(run_state.accumulator, 1);
        assert_eq!(run_state.program_counter, 0);
        assert_eq!(run_state.term_reason, 1);
    }

    #[test]
    pub fn test_program_autofix() {
        let test_program = vec![
            "acc +1".to_string(),
            "nop +0".to_string(),
            "jmp -2".to_string(),
        ];

        let fixed_program = program_autofix(&test_program);

        assert_eq!(fixed_program.0.len(), test_program.len());
        assert_eq!(fixed_program.0[2],"nop +0".to_string());
    }
}