use crate::player::{Player};

pub fn option_to_string(opt: &Option<Player>) -> String{
    match opt {
        Some(Player::O) => return String::from("  O  "),
        Some(Player::X) => return String::from("  X  "),
        None => return String::from("     "),
    }

}