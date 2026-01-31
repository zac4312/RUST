mod controller;
mod user_input;

use crate::controller::Action;
use crate::controller::AppStats;

fn main() {
    
    let mut stats = AppStats::build_app();
    let play = String::from("y");

    while play == "y" {
        let todo = user_input::choose_act(); 

        let action = match todo.as_str() {
            "1" => Action::Add,
            "2" => Action::List,
            "4" => Action::ListId,
            _ => Action::Quit, 
        };

        controller::do_action(action, &mut stats);
    }

}




