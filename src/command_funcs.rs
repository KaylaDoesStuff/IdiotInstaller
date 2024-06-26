use crate::constant_vars::FILES;
use crate::constant_vars::TERMINAL;
use crate::constant_vars::BROWSER;
use crate::constant_vars::MEDIA;
use crate::constant_vars::GAMING;
use crate::constant_vars::CONNECTION;
use crate::constant_vars::DESKTOP;
use crate::constant_vars::EXTRA;
use crate::constant_vars::GAMES;
use crate::constant_vars::EMULATORS;
use std::process::Command;
use std::io;

 
pub fn check_distro() -> String {
    let mut _distro: &str = "Unknown";
    if Command::new("pacman").arg("-Q").arg("sudo").output().is_ok() {
        let sel = &mut _distro;
        *sel = "Arch";
    } else if Command::new("dpkg").arg("-s").arg("sudo").output().is_ok() {
        let sel = &mut _distro;
        *sel = "Debian";
    } else if Command::new("rpm").arg("-q").arg("sudo").output().is_ok() {
        let sel = &mut _distro;
        *sel = "RPM-Based";
    } else if Command::new("xbps-query").arg("-q").arg("sudo").output().is_ok() {
        let sel = &mut _distro;
        *sel = "Void";
    } else if Command::new("pkg").arg("info").arg("sudo").output().is_ok() {
        let sel = &mut _distro;
        *sel = "FreeBSD";
    } else if Command::new("emerge").arg("-q").arg("sudo").output().is_ok() {
        let sel = &mut _distro;
        *sel = "Gentoo";
    } else if Command::new("apk").arg("info").arg("sudo").output().is_ok() {
        let sel = &mut _distro;
        *sel = "Alpine";
    } else if Command::new("pmm").arg("-Q").arg("sudo").output().is_ok(){
        let sel = &mut _distro;
        *sel = "Bedrock";
    } else {
        let sel = &mut _distro;
        *sel = "Unknown";
    }
    return _distro.to_string();
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
        "games" => for i in GAMES {
            n += 1;
            println!("{n}) {i}");
            let sel = &mut user_selection;
            *sel = "{i}".to_string();
        },
        "emulators" => for i in EMULATORS {
            n += 1;
            println!("{n}) {i}");
            let sel = &mut user_selection;
            *sel = "{i}".to_string();
        },
        &_ => todo!(),
    }
    return user_selection;
}

pub fn get_input(question: &str) -> String {
    let mut user_input = String::new();
    println!("{question}");
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    return user_input;
}