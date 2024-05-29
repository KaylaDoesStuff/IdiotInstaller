use crate::constant_vars::PACMANS;
use crate::constant_vars::PACMANS_OUTPUTS;
use crate::constant_vars::BASE;
use crate::constant_vars::TYPE;
use crate::constant_vars::FILES;
use crate::constant_vars::TERMINAL;
use crate::constant_vars::BROWSER;
use crate::constant_vars::MEDIA;
use crate::constant_vars::GAMING;
use crate::constant_vars::CONNECTION;
use crate::constant_vars::DESKTOP;
use crate::constant_vars::EXTRA;
use std::process::Command;

 
pub fn check_distro() -> String {
    let which = "which".to_string();
    let mut distro = "Unknown";
    for (i, pacman) in PACMANS.iter().enumerate() {
        let check = command_input(which.clone(), pacman.to_string());
        if check.contains(PACMANS_OUTPUTS[i]) {
            let sel = &mut distro;
            *sel = BASE[i];
            break;
        }
    }
    return distro.to_string();
}

pub fn command_input(input: String, input2: String) -> String {
    let cmd = Command::new(format!("{input}"))
        .arg(format!("{input2}"))
        .output()
        .expect("Command Not Found");
    let output_utf8 = cmd.stdout;
    let output = String::from_utf8(output_utf8).expect("Not an argument");
    return output;
}

pub fn get_selection(install_type: &str) -> String {
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