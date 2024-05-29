use std::io;
use crate::command_funcs::check_distro;
use crate::command_funcs::get_selection;


mod constant_vars;
mod command_funcs;

fn main() {
    println!("Detected derivative of {}", check_distro());
    println!("What do you want to install?");
    let mut n = 0;
    for i in TYPE {
        n += 1;
        println!("{n}) {i}");
    }
    let mut selection = String::new();
    let _ = io::stdin().read_line(&mut selection);
    let selection = selection.trim().parse::<i32>().expect("Not a number.");
    match selection {
        1 => get_selection("files"),
        2 => get_selection("browsers"),
        3 => get_selection("terminal"),
        4 => get_selection("media"),
        5 => get_selection("gaming"),
        6 => get_selection("connection"),
        7 => get_selection("desktop"),
        8 => get_selection("extra"),
        i32::MIN..=0_i32 | 9_i32..=i32::MAX => todo!(),
    };
}


