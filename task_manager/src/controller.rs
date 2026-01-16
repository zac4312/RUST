mod util;
use util::*;
use std::io;

pub enum TaskState {
    Pending,
    Completed,
    Dropped,
    Unassigned,
}

pub fn set_state(task_state: &str) -> TaskState { 
    match task_state {
        "1" => TaskState::Pending,
        "2" => TaskState::Completed,
        "3" => TaskState::Dropped,
        
        _ => TaskState::Unassigned,
    }
}

pub enum Action {
  Add,
  Mark_As { id: usize, state: TaskState },
  List,
  Quit,
  Invalid,
}

pub fn do_action(act: Action) -> Task {
    match act {
        
       Action::Add => { Task::add() }
       
    }
}

pub struct Task{
   pub title: String,
   pub Id: usize,    
   pub state: TaskState,  
}

impl Task {

    pub fn build() -> Self {        
       Self {
            title: String::from("Task{}", count_tasks()),
            Id: util::count_tasks(),
            state: TaskState::Unassigned,
        }
    }

    pub fn add() {      
        let task = Self::build();
        util::store_tasks(task;
    }    
}

