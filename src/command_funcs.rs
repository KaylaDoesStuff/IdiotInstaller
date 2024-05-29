use crate::constant_vars::PACMANS;
use crate::constant_vars::PACMANS_OUTPUTS;
use crate::constant_vars::BASE;

 
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

pub fn commandInput(input: String, input2: String) -> String {
    let cmd = Command::new(format!("{input}"))
        .arg(format!("{input2}"))
        .output()
        .expect("Command Not Found");
    let output_utf8 = cmd.stdout;
    let output = String::from_utf8(output_utf8).expect("Not an argument");
    return output;