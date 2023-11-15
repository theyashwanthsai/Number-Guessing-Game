use std::io::{stdin};
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let target: i32 = rng.gen_range(0..1000);
    // let mut flag: bool = true;
    let mut flag = 10;
    println!("Guess the number! Range - 0 to 1000");
    
    while flag > 0 {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Input");
        let number: i32 = input.trim().parse().expect("Input not an integer");
        if number == target{
            println!("Correct Answer!");
            flag = 0;
        }
        else if number > target{
            flag -= 1;
            println!("High!, chances left {}", flag);
            
        }
        else if number < target{
            flag -= 1;
            println!("Less!, chances left {}", flag);
        }
    }
}
