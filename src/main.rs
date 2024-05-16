use std::io;
use std::process::Command;

const base: [&str; 7] = ["Arch", "Debian", "RHEL", "Gentoo", "OpenSUSE", "Void", "FreeBSD"]
const types: [&str; 3] = ["File Managers","Browsers","Extra"];
const file_managers: [&str; 4] = ["Caja", "Thunar", "Dolphin", "Nautilus"]

struct install_client {
    base: i32, 
    to_install: [],

}

fn main() {
    println!("What do you want to install?");
    for i in types {
        println!("{i}) {:?}", types.i);
    }
    let selection = io::stdin().read_line();
    match selection {
        (1) => get_selection("files");
    }
}

fn commands() {
    let mut mkdir = Command::new("mkdir");
    let mut pacman = Command::new("pacman");
    let mut apt = Command::new("apt");
    let mut rpm = Command::new("rpm");
}

fn get_selection(install_type: &str) -> String {
    println!("Please select a number");
    let i = 0;
    match install_type {
        ("files") => for i in file_managers {
            println!("{i}) {:?}", file_managers.i)
        }
        ("browsers") => for i in browsers {
            println!("{i}) {:?}", browsers.i)
        }
        ("base") => for i in base {
            println!("{i}) {:?}", base.i)
        }
    }

    return user_selection;
}
