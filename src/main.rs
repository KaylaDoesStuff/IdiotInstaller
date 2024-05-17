use std::io;
use std::process::Command;

const base: [&str; 7] = ["Arch", "Debian", "RHEL", "Gentoo", "OpenSUSE", "Void", "FreeBSD"];
const types: [&str; 3] = ["File Managers","Browsers","Extra"];
const file_managers: [&str; 4] = ["Caja", "Thunar", "Dolphin", "Nautilus"];
const browsers: [&str; 5] = ["Chrome", "Chromium", "Opera", "Firefox", "LibreWolf"];

fn main() {
    println!("What do you want to install?");
    let mut n = 0;
    for i in types {
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
    let _cmd = Command::new("bash").arg("-c");

    let _mkdir = Command::new("mkdir");
    let _pacman = Command::new("pacman");
    let _apt = Command::new("apt");
    let _rpm = Command::new("rpm");
}

fn get_selection(install_type: &str) -> String {
    println!("Please select a number");
    let mut n = 0;
    let mut user_selection = "1".to_string();
    match install_type {
        "files" => for i in file_managers {
            n += 1;
            println!("{n}) {i}");
            let sel = &mut user_selection;
            *sel = "{i}".to_string();
        },
        "browsers" => for i in browsers {
            n += 1;
            println!("{n}) {i}");
            let sel = &mut user_selection;
            *sel = "{i}".to_string();
        },
        "base" => for i in base {
            n += 1;
            println!("{n}) {i}");
            let sel = &mut user_selection;
            *sel = "{i}".to_string();
        },
        &_ => todo!(),
    }

    return user_selection;
}
