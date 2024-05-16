use std::io;
use std::process::Command;

const base: [&str; 7] = ["Arch", "Debian", "RHEL", "Gentoo", "OpenSUSE", "Void", "FreeBSD"];
const types: [&str; 3] = ["File Managers","Browsers","Extra"];
const file_managers: [&str; 4] = ["Caja", "Thunar", "Dolphin", "Nautilus"];

fn main() {
    println!("What do you want to install?");
    for i in types {
        println!("{i}\n");
    }
    let sel_choice: String = "";
    let selection = io::stdin().read_line(&mut sel_choice);
    match sel_choice {
        Ok(1) => get_selection("files"),
        Ok(2) => get_selection("browsers"),
        Ok(3) => get_selection("base")
    };
}

fn commands(command: &str, input: &str, input2: &str) {
    let mkdir = Command::new("mkdir")
        .arg("{input}")
        .arg("{input2}");
    let pacman = Command::new("pacman")
        .arg("{input}")
        .arg("{input2}");
    let apt = Command::new("apt")
        .arg("{input}")
        .arg("{input2}");
    let rpm = Command::new("rpm")
        .arg("{input}")
        .arg("{input2}");
}

fn get_selection(install_type: &str) -> String {
    println!("Please select a number");
    let i = 0;
    match install_type {
        "files" => for i in file_managers {
            println!("{i}\n")
        }
        "browsers" => for i in browsers {
            println!("{i}\n")
        }
        "base" => for i in base {
            println!("{i}\n")
        }
    }

    return user_selection;
}
