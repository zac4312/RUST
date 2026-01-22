#[derive(Debug)]
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

#[derive(Debug)]
pub enum Action {
  Add,
  MarkAs { id: usize, state: TaskState },
  List,
  Quit,
}

//pub fn do_action(act: Action) {
  //  match act { 
    //   Action::Add => Task::push_to_storage(),
      // _ => {}, 
    //}
//}

#[derive(Debug)]
pub struct Task {
   pub title: String,
   pub id: usize,    
   pub state: TaskState,  
}

impl Task {

    pub fn build_task(title: String, id: usize, state: TaskState) -> Self {         
       Self {
            title,
            id,
            state,
        }
    }


}

#[derive(Debug)]
pub struct AppState {
    pub storage: Vec<Task>,
    pub counter: usize,
}

impl AppState {   

    pub fn build_app() -> Self {
        Self {
            storage: Vec::new(),
            counter: 0,
        }
    }   

    pub fn count(&mut self) -> usize{    
        self.counter  += 1;
        self.counter
    }   
}



