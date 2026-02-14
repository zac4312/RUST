use std::io;
use crate::err_handling::Error;

pub fn choose_act() -> Result<u8, Error> { 
    println!("--------------------------------------------------------------");
    println!("--------------------------------------------------------------");
    println!("1: add task || 2: show tasks || 3: edit task state || 4: search by Id || 5: exit");
    let mut act_choice = String::new();
    io::stdin().read_line(&mut act_choice);
    
    act_choice.trim()
              .parse::<u8>()
              .map_err(|_| Error::InvalidInput)
}

pub fn choose_state() -> Result<u8, Error> {
    //set task state
    println!("Set Task STATE: ");
    let mut state_choice = String::new();
    io::stdin()
        .read_line(&mut state_choice);
        
    state_choice.trim()
                .parse::<u8>()
                .map_err(|_| Error::InvalidInput)
 }
 
pub fn set_title() -> String {
    //set task title
    println!("Title: ");
    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("set_state err"); 
   
    title.trim().to_string()
}

pub fn search_id() -> Result<usize, Error> {
    println!("Enter task Id: ");
    let mut id_choice = String::new();
    io::stdin()
        .read_line(&mut id_choice);
   
    id_choice.trim()
             .parse::<usize>()
             .map_err(|_| Error::InvalidInput)
}
