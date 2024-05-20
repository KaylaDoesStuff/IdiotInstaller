 pub fn check_distro() -> String {
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