use crate::user_input;
use crate::err_handling::Error;

#[derive(Debug)]
pub enum Action {
  Add,
  ListId, 
  EditTask,
  List,
  Invalid
}

pub fn do_action(act: Action, stats: &mut AppStats) -> Result<(), Error> {
  match act { 
    Action::Add => Ok(add_task_out(stats))?,
    Action::List =>  Ok(Task::show_tasks(stats)),
    Action::ListId => Ok(search_id_out(stats))?,
    Action::EditTask => Ok(edit_task_out(stats))?,
    Action::Invalid => Ok(invalid()),
  }
}

#[derive(Debug, Clone)]
pub enum TaskState {
    Pending,
    Completed,
    Dropped,
    Unassigned,
}

pub fn set_state(task_state: u8) -> Result<TaskState, Error> { 
    match task_state {
        1 => Ok(TaskState::Pending),
        2 => Ok(TaskState::Completed),
        3 => Ok(TaskState::Dropped),
        
        _ => { Err::<TaskState, Error>(set_default_state()); Ok(TaskState::Unassigned) },
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
 
    pub fn add_task(title: String, id: usize, state: &u8, stats: &mut AppStats) -> Result<(), Error> {
        let task = Self::build_task(title, id, set_state(*state)?);
        stats.storage.push(task);
        Ok(())
    } 

    pub fn show_tasks(stats: &AppStats) { println!("{:?}", stats.storage); } 
     
    pub fn search_by_id(target_id: usize, stats: &mut AppStats) -> Result<&mut Task, Error> { 
        match stats.storage.iter_mut().find(|task| task.id == target_id) {
            Some(task) => Ok(task),
            None => Err(Error::TaskNotFound),
        }    
    }

    pub fn edit_task(title: String, state: u8, target_id: usize, stats: &mut AppStats) -> Result<(), Error> {
         
       let found_task: &mut Task = Self::search_by_id(target_id, stats)?; 
            println!("Editing Task: {:?}", found_task); 

            found_task.title = title;
            found_task.state = set_state(state)?;
            Ok(())
    }
}
//assosciated fn()

pub fn search_id_out(stats: &mut AppStats) -> Result<(), Error> {
   let target_id =  user_input::search_id()?; 
   let searchId = Task::search_by_id(target_id, stats);
   println!("{:?}", searchId);
   Ok(())
}

pub fn  edit_task_out(stats: &mut AppStats) -> Result<(), Error> {
    let target_id = user_input::search_id()?; 
    Task::search_by_id(target_id, stats)?;
    let mut title = user_input::set_title();
    if title.is_empty() {title = format!("task{}", stats.counter);}
    
    let state = user_input::choose_state()?;
    let editTask = Task::edit_task(title, state, target_id, stats);

    println!("{:?}", editTask);
    Ok(())
}

pub fn add_task_out(stats: &mut AppStats) -> Result<(), Error> { 
    let mut title = user_input::set_title();
    if title.is_empty() { title = format!("task{}", stats.count()); }

    let id = stats.counter;
   
    let state = user_input::choose_state()?;
     
    let task = Task::add_task(title, id, &state, stats);
    println!("Task Saved: {:?}", stats.storage);
     Ok(())
} 

pub fn invalid() { println!("{:?}", Error::InvalidInput); }
pub fn set_default_state() -> Error {println!("Due to: {:?} State is Unnasigned", Error::InvalidInput); Error::InvalidInput}
