use std::io;
use std::process::Command;

const BASE: [&str; 7] = ["Arch", "Debian", "RHEL", "Gentoo", "OpenSUSE", "Void", "FreeBSD"];
const TYPE: [&str; 3] = ["File Managers","Browsers","Extra"];
const FILES: [&str; 4] = ["Caja", "Thunar", "Dolphin", "Nautilus"];
const BROWSER: [&str; 5] = ["Chrome", "Chromium", "Opera", "Firefox", "LibreWolf"];
const TERMINAL: [&str; 5] = ["Alacritty", "Kitty", "XTerm", "Konsole", "GNOME"];

fn main() {
    println!("What do you want to install?");
    let mut n = 0;
    for i in TYPE {
        n += 1;
        println!("{n}) {i}");
    }
    let mut selection = String::new();
    io::stdin().read_line(&mut selection).unwrap();
    let selection: u32 = selection.parse::<u32>().expect("Not a number.");
    match selection {
        1 => get_selection("files"),
        2 => get_selection("browsers"),
        3 => get_selection("bases"),
        0_u32 | 4_u32..=u32::MAX => todo!(),
    };
}

fn commands() {
    let _cmd = Command::new("bash")
        .arg("-c")
        .arg("{input}");
}

fn get_selection(install_type: &str) -> String {
    println!("Please select a number");
    let mut n = 0;
    let mut user_selection = "1".to_string();
    match install_type {
        "files" => for i in FILES{
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
        "base" => for i in BASE {
            n += 1;
            println!("{n}) {i}");
            let sel = &mut user_selection;
            *sel = "{i}".to_string();
        },
        "terminals" => for i in TERMINAL {
            n += 1;
            println!("{n}) {i}");
            let sel = &mut user_selection;
            *sel = "{i}".to_string();
        },
        &_ => todo!(),
    }
    return user_selection;
}
