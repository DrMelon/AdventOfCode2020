use cursive::{Cursive, CursiveExt};
use cursive::views::{Dialog, TextView, SelectView};
use cursive::traits::*;
use cursive_async_view::{AsyncView, AsyncState};
use std::thread;
use std::time::Duration;

pub mod day1;

fn main()  {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    // Show AoC list
    let menu = SelectView::<i32>::new()
        .on_submit(menu_selection)
        .with_name("main_menu")
        .fixed_size((50,20));

    
    
  

    siv.add_layer(Dialog::around(menu).title("Main Menu"));
    populate_menu(&mut siv);

    // Show intro dialogue.
    siv.add_layer(Dialog::around(TextView::new("🎄 Welcome to my Advent of Code 2020 solutions! ~@drmelon 🍉"))
                         .title("🎄 AoC 2020 🎄")
                         .button("Thanks!", |s| {s.pop_layer();}));

    // Starts the event loop.
    siv.run();    
}

fn menu_selection(s: &mut Cursive, selection: &i32) {    
    match selection {
        0 => {
            day1::display_day_menu(s);
        }
        999 => {
            s.quit();
        },
        _ => {}
    }
}

fn populate_menu(s: &mut Cursive) {
    s.call_on_name("main_menu", |view: &mut SelectView<i32>| {
        view.add_item("Day 1) Report Repair", 0);
        view.add_item("Quit", 999);
    });
}