use std::io;
use std::process::Command;
use crate::command_funcs;
use crate::consts::BASE;
use crate::consts::TYPE;
use crate::consts::FILES;
use crate::consts::TERMINAL;
use crate::consts::BROWSER;
use crate::consts::MEDIA;
use crate::consts::GAMING;
use crate::consts::CONNECTION;
use crate::consts::DESKTOP;
use crate::consts::EXTRA;
use crate::consts::PACMANS;
use crate::consts::PACMANS_OUTPUTS;



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

fn commandInput(input: String, input2: String) -> String {
    let cmd = Command::new(format!("{input}"))
        .arg(format!("{input2}"))
        .output()
        .expect("Command Not Found");
    let output_utf8 = cmd.stdout;
    let output = String::from_utf8(output_utf8).expect("Not an argument");
    return output;


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
