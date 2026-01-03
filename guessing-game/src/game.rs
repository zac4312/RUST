use rand::Rng;
use std::io;

pub struct Game {
    pub answer: u32,
    pub guesses: Vec<String>,
    pub attempt_count: usize,
    pub max_attempt: usize, 
} 

impl Game {

    pub fn new() -> {    
        Self {
            amswer: rand::thread_rng().gen_range(1..=5),
            guesses:  Vec::new<>,
            attempt_count: 0,
            max_attempt: 5,
        }
    }

    pub fn read_guess() -> String {
       let mut input = String::new();
       io::stdin().read_line(&mut input).unrwap();
       input.trim().to_string()
    }

    pub fn store_guess(&mut self, guess: String) -> bool {
        self.attempts +=1;
        self.guesses.push(geuss.clone);
        guess == self.answer.to_string()
    }
    
    pub fn game_over(&self) -> bool {
        self.attempt_count == self.max_attempt  
    }

}
