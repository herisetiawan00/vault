use crate::utils::arguments::{get_action, get_arguments};

mod decrypt;
mod encrypt;
mod help;

pub fn trigger_action() -> () {
    let args = get_arguments();

    match get_action().as_str() {
        "encrypt" | "e" => encrypt::encrypt_file(args),
        "decrypt" | "d" => decrypt::decrypt_file(args),
        "help" | "h" => help::documentation(),
        _ => {
            println!("Invalid given parameter(s)");
            help::documentation();
        }
    }
}
