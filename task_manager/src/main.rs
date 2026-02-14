mod controller;
mod user_input;
mod err_handling;
use crate::controller::Action;
use crate::controller::AppStats;

fn main() {
    
    let mut stats = AppStats::build_app();

    loop {
        let todo = user_input::choose_act();
        
        let action = match todo {
            Ok(1) => Action::Add,
            Ok(2) => Action::List,
            Ok(3) => Action::EditTask, 
            Ok(4) => Action::ListId,
            Ok(5) => break,

            Ok(_) => Action::Invalid,
            Err(_) => Action::Invalid, 
        };
        if let Err(e) = controller::do_action(action, &mut stats) {
            println!("Error: {:?}", e);
        } 
    }

}




