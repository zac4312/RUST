use std::io;

struct Calculator{
    num1: i64,
    num2: i64,
}

impl Calculator {
  
  fn add(&self) -> i64 { self.num1 + self.num2 }
  fn minus(&self) -> i64 { self.num1 - self.num2 }
  fn multiply(&self) -> i64 { self.num1 * self.num2 }
  fn divide(&self) -> i64 { self.num1 / self.num2 } 
  } 

fn input() -> i64 { 

    println!("Enter Number: ");
    let mut number = String::new();  
    io::stdin().read_line(&mut number).expect("Invalid Input");
    number.trim().parse::<i64>().expect("parse err")

    }


fn main() {

    let mut calc = Calculator {num1: 0, num2: 0,};

    let state = "y";
    while state == "y" {

    calc.num1 = input();
    calc.num2 = input();

     let mut choice = String::new();
    println!(" | + | - | x | / | ");
    io::stdin().read_line(&mut choice)
               .expect("error");   

    println!("{}", calc.num1);
    println!("{}", calc.num2);

    match choice.trim() { 
       "+" => println!("{}", calc.add()),
       "-" => println!("{}", calc.minus()), 
       "x" => println!("{}", calc.multiply()), 
       "/" => println!("{}", calc.divide()),
       _ => println!("Invalid iput"),
    }

    }
} 

