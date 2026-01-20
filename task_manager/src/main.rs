mod controller;
use crate::controller::Task;
use crate::controller::AppState;
use crate::controller::TaskState;

use std::io;

fn main() {
 
let repeat = "y";
while repeat == "y" {

 let mut b = AppState::build_app();
    //set task state
    println!("Set Task STATE: ");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("set_state err");

    //set task title
    println!("Title: ");
    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("set_state err");

    if  title.trim().is_empty() {
        title = format!("task{}", b.count());
    }
   
    let a = Task::build_task(title, b.count(), controller::set_state(&choice));

    println!("{:?}", a);
    println!("{:?}", b);

}
}



