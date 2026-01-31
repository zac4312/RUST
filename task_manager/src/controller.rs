use crate::user_input;

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
  ListId, 
  MarkAs,
  Quit,
  List,
}

pub fn do_action(act: Action, stats: &mut AppStats) {
  match act { 
    Action::Add => Task::add_task(stats),
    Action::List => Task::show_tasks(stats),
    Action::ListId => Task::search_by_id(stats),
    Action::Quit => Task::quit(stats),
    _ => {}, 
  }
}

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

    pub fn add_task(stats: &mut AppStats) {
        let mut title = user_input::set_title();
        if title.is_empty() {title = format!("task{}", stats.counter);}

        let state = user_input::choose_state();
        let id = stats.count();
        let task = Self::build_task(title, id, set_state(&state));
        println!("Saved task: {:?}", task);
        stats.storage.push(task);
    } 

    pub fn show_tasks(stats: &AppStats) { println!("{:?}", stats.storage); } 
    
    pub fn quit(stats: &AppStats) { println!("{:?}", stats.storage); println!("quitting..."); }  

    pub fn search_by_id(stats: &AppStats) {
        let target_id = user_input::search_id(); 
        let stored_tasks = &stats.storage;

        for task in stored_tasks.iter() {            
            if task.id == target_id {
                println!("{:?}", task);
            }
        }    
    }
} 

#[derive(Debug)]
pub struct AppStats {
    pub storage: Vec<Task>,
    pub counter: usize,
}

impl AppStats {   

    pub fn build_app() -> Self {
        Self {
            storage: Vec::new(),
            counter: 0,
        }
    }   

    pub fn count(&mut self) -> usize {    
        self.counter  += 1;
        self.counter
    }   

}



