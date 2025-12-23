use std::io;
use rand::Rng;

fn main() { 
   
    let mut ans: Vec<String> = Vec::new();

    let mut choice = String::from("y"); 
    io::stdin().read_line(&mut choice)
               .expect("wrong input"); 
    
    while choice.trim() == "y" {
        
        println!("-------------------------------------"); 
        println!("Guess a number from 1-5!");
        println!("You got 5 tries!!"); 
        
        let num = rand::thread_rng().gen_range(1..=5); 
        let str_num = num.to_string(); println!("Enter a guess:"); 
        
        let mut guess = String::new();
        io::stdin().read_line(&mut guess) 
                   .expect("Wrong guess"); 
    
        if guess.trim() == str_num {
            println!("Corrrect!"); 
            println!("You guess: {}", guess.trim()); 
            println!("Answer: {str_num}");
        } else { println!("Incorroce!"); 
                 println!("The number was: {str_num}"); 
               } 
 
        ans.push(guess);
        
        println!("Again? (y/n): "); 
        choice.clear(); 
        io::stdin().read_line(&mut choice) 
                   .expect("wrong input");
        } 

    println!("end"); 
    println!("Guesses:"); 
    println!("{:?}", ans); 
}

