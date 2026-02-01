use crate::user_input;

#[derive(Debug, Clone)]
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
  EditTask,
  List,
  Invalid,
}

pub fn do_action(act: Action, stats: &mut AppStats) {
  match act { 
    Action::Add => Task::add_task(stats),
    Action::List => Task::show_tasks(stats),
    Action::ListId => Task::search_by_id(stats),
    Action::EditTask => Task::edit_task(stats), 
    Action::Invalid => Task::invalid_option(),
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

    pub fn search_by_id(stats: &AppStats) {
        let target_id = user_input::search_id();  

        for task in stats.storage.iter() {            
            if task.id == target_id {
                println!("{:?}", task);
                break;
            } else {println!("Please enter the task ID"); break;}
        }    
    }

    pub fn edit_task(stats: &mut AppStats) {
        let target_id = user_input::search_id(); 

        for task in stats.storage.iter_mut() {            
            if task.id == target_id { 
                println!("Editing Task: {:?}", task); 

                task.title = user_input::set_title();
                if task.title.is_empty() {task.title = format!("task{}", task.id);}

                let ref_state = user_input::choose_state();
                task.state = set_state(&ref_state);
                println!("Updtated Task: {:?}", task);
                break;
            } else {println!("Please enter the task ID"); break;} 
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



