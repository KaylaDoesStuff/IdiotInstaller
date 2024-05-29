 mod consts;
 
 pub fn check_distro() -> String {
    let which = "which".to_string();
    let mut distro = "Unknown";
    for (i, pacman) in consts::PACMANS.iter().enumerate() {
        let check = commandInput(which.clone(), pacman.to_string());
        if check.contains(consts::PACMANS_OUTPUTS[i]) {
            let sel = &mut distro;
            *sel = consts::BASE[i];
            break;
        }
    }
    return distro.to_string();
}