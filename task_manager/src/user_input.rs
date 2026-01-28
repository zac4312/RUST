use std::io;

pub fn choose_act() -> String {
    println!("--------------------------------------------------------------");
    println!("--------------------------------------------------------------");
    println!("1: add task || 2: show tasks || 3: edit task state || 4: search by Id || 5: exit");
    let mut act_choice = String::new();
    io::stdin()
        .read_line(&mut act_choice)
        .expect("act choice err");

    act_choice.trim().to_string()
}

pub fn choose_state() -> String {
    //set task state
    println!("Set Task STATE: ");
    let mut state_choice = String::new();
    io::stdin()
        .read_line(&mut state_choice)
        .expect("set_state err");

    state_choice.trim().to_string()
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

pub fn search_id() -> usize {
    println!("Enter task Id: ");
    let mut id_choice = String::new();
    io::stdin()
        .read_line(&mut id_choice)
        .expect("set_state err");
   
   id_choice.parse::<usize>()
        .expect("Id parsing err")
}
