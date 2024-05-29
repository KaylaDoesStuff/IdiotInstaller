use std::io;
use std::process::Command;

mod consts;
mod commands;


fn main() {
    println!("Detected derivative of {}", commands::check_distro());
    println!("What do you want to install?");
    let mut n = 0;
    for i in consts::TYPE {
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

fn commands(input: String, input2: String) -> String {
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
        "files" => for i in consts::FILES {
            n += 1;
            println!("{n}) {i}");
            let sel = &mut user_selection;
            *sel = "{i}".to_string();
        },
        "browsers" => for i in consts::BROWSER {
            n += 1;
            println!("{n}) {i}");
            let sel = &mut user_selection;
            *sel = "{i}".to_string();
        },
        "terminal" => for i in consts::TERMINAL {
            n += 1;
            println!("{n}) {i}");
            let sel = &mut user_selection;
            *sel = "{i}".to_string();
        },
        "media" => for i in consts::MEDIA {
            n += 1;
            println!("{n}) {i}");
            let sel = &mut user_selection;
            *sel = "{i}".to_string();
        },
        "gaming" => for i in consts::GAMING {
            n += 1;
            println!("{n}) {i}");
            let sel = &mut user_selection;
            *sel = "{i}".to_string();
        },
        "connection" => for i in consts::CONNECTION {
            n += 1;
            println!("{n}) {i}");
            let sel = &mut user_selection;
            *sel = "{i}".to_string();
        },
        "desktop" => for i in consts::DESKTOP {
            n += 1;
            println!("{n}) {i}");
            let sel = &mut user_selection;
            *sel = "{i}".to_string();
        },
        "extra" => for i in consts::EXTRA {
            n += 1;
            println!("{n}) {i}");
            let sel = &mut user_selection;
            *sel = "{i}".to_string();
        },
        &_ => todo!(),
    }
    return user_selection;
}
