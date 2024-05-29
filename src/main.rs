use std::io;
use std::process::Command;
use crate::command_funcs::check_distro;
use crate::constant_vars::TYPE;
use crate::constant_vars::FILES;
use crate::constant_vars::TERMINAL;
use crate::constant_vars::BROWSER;
use crate::constant_vars::MEDIA;
use crate::constant_vars::GAMING;
use crate::constant_vars::CONNECTION;
use crate::constant_vars::DESKTOP;
use crate::constant_vars::EXTRA;

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

fn get_selection(install_type: &str) -> String {
    println!("Please select a number");
    let mut n = 0;
    let mut user_selection = "1".to_string();
    match install_type {
        "files" => for i in FILES {
            n += 1;
            println!("{n}) {i}");
            let sel = &mut user_selection;
            *sel = "{i}".to_string();
        },
        "browsers" => for i in BROWSER {
            n += 1;
            println!("{n}) {i}");
            let sel = &mut user_selection;
            *sel = "{i}".to_string();
        },
        "terminal" => for i in TERMINAL {
            n += 1;
            println!("{n}) {i}");
            let sel = &mut user_selection;
            *sel = "{i}".to_string();
        },
        "media" => for i in MEDIA {
            n += 1;
            println!("{n}) {i}");
            let sel = &mut user_selection;
            *sel = "{i}".to_string();
        },
        "gaming" => for i in GAMING {
            n += 1;
            println!("{n}) {i}");
            let sel = &mut user_selection;
            *sel = "{i}".to_string();
        },
        "connection" => for i in CONNECTION {
            n += 1;
            println!("{n}) {i}");
            let sel = &mut user_selection;
            *sel = "{i}".to_string();
        },
        "desktop" => for i in DESKTOP {
            n += 1;
            println!("{n}) {i}");
            let sel = &mut user_selection;
            *sel = "{i}".to_string();
        },
        "extra" => for i in EXTRA {
            n += 1;
            println!("{n}) {i}");
            let sel = &mut user_selection;
            *sel = "{i}".to_string();
        },
        &_ => todo!(),
    }
    return user_selection;
}
