mod controller;
mod user_input;

use crate::controller::Action;
use crate::controller::AppStats;

fn main() {
    
    let mut stats = AppStats::build_app();

    loop {
        let todo = user_input::choose_act(); 

        let action = match todo.as_str() {
            "1" => Action::Add,
            "2" => Action::List,
            "3" => Action::EditTask, 
            "4" => Action::ListId,
            "5" => break,
            _ => Action::Invalid,
        };

        controller::do_action(action, &mut stats);
    }

}




