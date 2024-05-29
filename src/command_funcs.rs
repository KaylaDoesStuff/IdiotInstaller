 mod constant_vars;
 mod main;
 
 pub fn check_distro() -> String {
    let which = "which".to_string();
    let mut distro = "Unknown";
    for (i, pacman) in constant_vars::PACMANS.iter().enumerate() {
        let check = main::commandInput(which.clone(), pacman.to_string());
        if check.contains(constant_vars::PACMANS_OUTPUTS[i]) {
            let sel = &mut distro;
            *sel = constant_vars::BASE[i];
            break;
        }
    }
    return distro.to_string();
}