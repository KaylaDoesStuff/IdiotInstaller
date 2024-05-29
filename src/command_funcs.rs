use crate::constant_vars::PACMANS;
use crate::constant_vars::PACMANS_OUTPUTS;
use crate::constant_vars::BASE;
use main::commandInput;

 
 pub fn check_distro() -> String {
    let which = "which".to_string();
    let mut distro = "Unknown";
    for (i, pacman) in PACMANS.iter().enumerate() {
        let check = commandInput(which.clone(), pacman.to_string());
        if check.contains(PACMANS_OUTPUTS[i]) {
            let sel = &mut distro;
            *sel = BASE[i];
            break;
        }
    }
    return distro.to_string();
}