use std::io;
use std::process::Command;

const BASE: [&str; 7] = ["Arch", "Debian", "Gentoo", "FreeBSD", "Alpine", "Void", "RHEL"];
const TYPE: [&str; 8] = ["File Managers","Browsers","Terminals", "Media", "Gaming", "Connection Utilities", "Desktop Environments", "Extras"];
const FILES: [&str; 4] = ["Caja", "Thunar", "Dolphin", "Nautilus"];
const BROWSER: [&str; 6] = ["Chrome", "Chromium", "Opera", "Firefox", "LibreWolf", "Edge"];
const TERMINAL: [&str; 5] = ["Alacritty", "Kitty", "XTerm", "Konsole", "GNOME"];
const EXTRA: [&str; 10] = ["Paru", "Yay", "Pamac", "Connman", "Blueman", "NetworkManager", "Hyfetch", "Htop", "Cmatrix", "Papirus Icons"];
const GAMING: [&str; 7] = ["Steam", "Discord", "Prism Launcher", "ATLauncher", "Heroic Launcher", "Lutris", "ProtonUp-QT"];
const MEDIA: [&str; 7] = ["Spotify", "VLC", "ThunderBird", "OBS Studio", "KdenLive", "DaVinci Resolve", "Rhythm Box"];
const DESKTOP: [&str; 8] = ["KDE Plasma", "GNOME", "Xfce4", "LXQT", "MATE", "i3", "Bspwm", "Sway"];
const CONNECTION: [&str; 8] = ["Moonlight", "Sunshine", "Haguichi(Hamachi)", "ZeroTier", "WireGuard", "OpenSSH", "Anydesk", "TeamViewer"];
const PACMANS: [&str; 7] = ["pacman", "apt", "emerge", "pkg", "apk", "xpvs", "dnf"];
const PACMANS_OUTPUTS: [&str; 7] = ["/usr/bin/pacman", "/usr/bin/apt", "/usr/bin/emerge", "/usr/bin/pkg", "/usr/bin/apk", "/usr/bin/xpvs", "/usr/bin/dnf"];

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

fn check_distro() -> String {
    let which = "which".to_string();
    let mut distro = "Unknown";
    for (i, pacman) in PACMANS.iter().enumerate() {
        let check = commands(which.clone(), pacman.to_string());
        if check.contains(PACMANS_OUTPUTS[i]) {
            let sel = &mut distro;
            *sel = BASE[i];
            break;
        }
    }
    return distro.to_string();
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
            println!("{n}) {i}")
            let sel = &mut user_selection;
            *sel = "{i}".to_string();
        },
        &_ => todo!(),
    }
    return user_selection;
}
