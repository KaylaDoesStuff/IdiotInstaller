use std::io;
use std::process::Command;

const BASE: [&str; 7] = ["Arch", "Debian", "RHEL", "Gentoo", "Void", "FreeBSD", "Alpine"];
const TYPE: [&str; 7] = ["File Managers","Browsers","Terminals", "Media", "Gaming", "Connection Utilities", "Desktop Environments"];
const FILES: [&str; 4] = ["Caja", "Thunar", "Dolphin", "Nautilus"];
const BROWSER: [&str; 6] = ["Chrome", "Chromium", "Opera", "Firefox", "LibreWolf", "Edge"];
const TERMINAL: [&str; 5] = ["Alacritty", "Kitty", "XTerm", "Konsole", "GNOME"];
const EXTRA: [&str; 13] = ["Paru", "Yay", "Pamac", "Connman", "Blueman", "NetworkManager", "Hyfetch", "Htop", "Cmatrix", "Papirus Icons", "OpenSSH", "Anydesk", "TeamViewer"];
const GAMING: [&str; 7] = ["Steam", "Discord", "Prism Launcher", "AT Launcher", "Heroic Launcher", "Lutris", "ProtonUp-QT"];
const MEDIA: [&str; 7] = ["Spotify", "VLC", "ThunderBird", "OBS Studio", "KdenLive", "DaVinci Resolve", "Rhythm Box"];
const DESKTOP: [&str; 8] = ["KDE Plasma", "GNOME", "Xfce4", "LXQT", "MATE", "i3", "Bspwm", "Sway"];
const CONNECTION: [&str; 5] = ["Moonlight", "Sunshine", "Haguichi(Hamachi)", "ZeroTier", "WireGuard"];
const PACMANS: [&str; 9] = ["pacman", "apt", "zypper", "emeerge", "pkg", "apk", "xpvs", "dnf", "yum"];

fn main() {
    let distribution = check_distro();
    println!("What do you want to install?");
    let mut n = 0;
    for i in TYPE {
        n += 1;
        println!("{n}) {i}");
    }
    let mut selection = String::new();
    let _ = io::stdin().read_line(&mut selection);
    let selection = selection.trim().parse::<u32>().expect("Not a number.");
    match selection {
        1 => get_selection("files"),
        2 => get_selection("browsers"),
        3 => get_selection("terminal"),
        4 => get_selection("media"),
        5 => get_selection("gaming"),
        6 => get_selection("connection"),
        7 => get_selection("desktop"),
        0_u32 | 4_u32..=u32::MAX => todo!(),
    };
}

fn check_distro() -> String {
    let which = "which".to_string();
    let mut distro = "Unknown".to_string();
    for i in PACMANS {
        let check = commands(&which, &i.to_string());
        match check {
            "/usr/bin/pacman" => {
                let distro_sel = &mut distro;
                *distro_sel = "Arch".to_string();
            },
            "/usr/bin/apt" => {
                let distro_sel = &mut distro;
                *distro_sel = "Debian".to_string();
            },
            "/usr/bin/dnf" => {
                let distro_sel = &mut distro;
                *distro_sel = "RHEL".to_string();
            },
            "/usr/bin/emerge" => {
                let distro_sel = &mut distro;
                *distro_sel = "Gentoo".to_string();
            },
            "/usr/bin/pkg" => {
                let distro_sel = &mut distro;
                *distro_sel = "FreeBSD".to_string();
            },
            "/usr/bin/apk" => {
                let distro_sel = &mut distro;
                *distro_sel = "Alpine".to_string();
            },
            "/usr/bin/xpvs" => {
                let distro_sel = &mut distro;
                *distro_sel = "Void".to_string();
            },
            &_ => todo!(),
        }
    }
    return distro;
}

fn commands(input: &String, input2: &String) -> &str {
    let cmd = Command::new("bash")
        .arg("-c")
        .arg("{input}")
        .arg("{input2}")
        .output()
        .expect("Command Not Found");
    let output_utf8 = cmd.stdout;
    let output = output_utf8.String::from_utf(), { return output };
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
        &_ => todo!(),
    }
    return user_selection;
}
