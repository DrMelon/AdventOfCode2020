use cursive::traits::*;
use cursive::views::{Dialog, SelectView, TextView, ScrollView};
use cursive::Cursive;

use cursive_async_view::AsyncView;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;

fn main() {
    // Creates the cursive root - required for every application.
    let mut siv =  Cursive::new(|| {
        let crossterm_backend = cursive::backends::crossterm::Backend::init().unwrap();
        let buffered_backend = cursive_buffered_backend::BufferedBackend::new(crossterm_backend);
        Box::new(buffered_backend)
    });

    // Load theme
    siv.load_theme_file("config/xmas.toml").unwrap();

    // Show AoC list
    let menu = SelectView::<i32>::new()
        .on_submit(menu_selection)
        .with_name("main_menu")
        .fixed_size((50, 20));

    siv.add_layer(Dialog::around(ScrollView::new(menu)).title("Main Menu"));
    populate_menu(&mut siv);

    // Show intro dialogue.
    siv.add_layer(
        Dialog::around(TextView::new(
            "🎄 Welcome to my Advent of Code 2020 solutions! ~@drmelon 🍉",
        ))
        .title("🎄 AoC 2020 🎄")
        .button("Thanks!", |s| {
            s.pop_layer();
        }),
    );

    // Starts the event loop.
    siv.run();
}

fn menu_selection(s: &mut Cursive, selection: &i32) {
    match selection {
        0 => {
            day1::display_day_menu(s);
        }
        1 => {
            day2::display_day_menu(s);
        }
        2 => {
            day3::display_day_menu(s);
        }
        3 => {
            day4::display_day_menu(s);
        }
        4 => {
            day5::display_day_menu(s);
        }
        5 => {
            day6::display_day_menu(s);
        }
        6 => {
            day7::display_day_menu(s);
        }
        7 => {
            day8::display_day_menu(s);
        }
        8 => {
            day9::display_day_menu(s);
        }
        9 => {
            day10::display_day_menu(s);
        }
        10 => {
            day11::display_day_menu(s);
        }
        11 => {
            day12::display_day_menu(s);
        }
        999 => {
            s.quit();
        }
        _ => {}
    }
}

fn populate_menu(s: &mut Cursive) {
    s.call_on_name("main_menu", |view: &mut SelectView<i32>| {
        view.add_item("Day  1) Report Repair 📄", 0);
        view.add_item("Day  2) Password Philosophy 🤫", 1);
        view.add_item("Day  3) Toboggan Trajectory 🛷", 2);
        view.add_item("Day  4) Passport Processing 📕", 3);
        view.add_item("Day  5) Binary Boarding 💺", 4);
        view.add_item("Day  6) Custom Customs 🛅", 5);
        view.add_item("Day  7) Handy Haversacks 🧳", 6);
        view.add_item("Day  8) Handheld Halting 🎮", 7);
        view.add_item("Day  9) Encoding Error ⛔", 8);
        view.add_item("Day 10) Adapter Array ⚡", 9);
        view.add_item("Day 11) Seating System 🪑", 10);
        view.add_item("Day 12) Rain Risk ☔", 11);
        view.add_item("Quit", 999);
    });
}

