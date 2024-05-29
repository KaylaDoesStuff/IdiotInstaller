use crate::PACMANS;
use crate::PACMANS_OUTPUTS;
use crate::BASE;
use crate::commandInput;

 
 pub fn check_distro() -> String {
    let which = "which".to_string();
    let mut distro = "Unknown";
    for (i, pacman) in PACMANS.iter().enumerate() {
        let check = main::commandInput(which.clone(), pacman.to_string());
        if check.contains(PACMANS_OUTPUTS[i]) {
            let sel = &mut distro;
            *sel = constant_vars::BASE[i];
            break;
        }
    }
    return distro.to_string();
}