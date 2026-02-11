use crate::user_input;
use crate::err_handling::Error;

#[derive(Debug)]
pub enum Action {
  Add,
  ListId, 
  EditTask,
  List,
  Invalid,
}

pub fn do_action(act: Action, stats: &mut AppStats) {
  match act { 
    Action::Add => Task::add_task(stats),
    Action::List => Task::show_tasks(stats),
    Action::ListId => search_id_out(stats),
    Action::EditTask => edit_task_out(stats), 
    Action::Invalid => Task::invalid_option(),
  };
}

#[derive(Debug, Clone)]
pub enum TaskState {
    Pending,
    Completed,
    Dropped,
    Unassigned,
}

pub fn set_state(task_state: u8) -> TaskState { 
    match task_state {
        1 => TaskState::Pending,
        2 => TaskState::Completed,
        3 => TaskState::Dropped,
        
        _ => TaskState::Unassigned,
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

#[derive(Debug, Clone)]
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
        let id = stats.count();

        let mut title = user_input::set_title();
        if title.is_empty() {title = format!("task{}", stats.counter);}

        let state = user_input::choose_state(); 


        let task = Self::build_task(title, id, set_state(&state));
        println!("Saved task: {:?}", task);
        stats.storage.push(task);
    } 

    pub fn show_tasks(stats: &AppStats) { println!("{:?}", stats.storage); } 
    
    pub fn invalid_option() {println!("Invalid Option");}

    pub fn search_by_id(target_id: usize, stats: &mut AppStats) -> Result<&mut Task, Error> { 
        match stats.storage.iter_mut().find(|task| task.id == target_id) {
            Some(task) => Ok(task),
            None => Err(Error::TaskNotFound),
        }    
    }

    pub fn edit_task(target_id: usize, stats: &mut AppStats) -> Result<(), Error> {
         
       let found_task: &mut Task = Self::search_by_id(target_id, stats)?; 
            println!("Editing Task: {:?}", found_task); 

            found_task.title = user_input::set_title();
            if found_task.title.is_empty() {found_task.title = format!("task{}", found_task.id);}

            let ref_state = user_input::choose_state();
            found_task.state = set_state(&ref_state);
            println!("Updtated Task: {:?}", found_task);
            Ok(())
    } 
} 

//assosciated fn()

pub fn search_id_out(stats: &mut AppStats) {
   let target_id = user_input::search_id(); 
   let searchId_out = Task::search_by_id(target_id, stats);
   println!("{:?}", searchId_out);
}

pub fn  edit_task_out(stats: &mut AppStats) {
   let target_id = user_input::search_id(); 
   let edit_out = Task::edit_task(target_id, stats);
   println!("{:?}", edit_out); 
} 




