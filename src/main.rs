use std::io::{stdin};
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let target: i32 = rng.gen_range(0..1000);
    let mut flag: bool = true;
    println!("Guess the number! Range - 0 to 1000");
    
    while(flag){
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Input");
        let number: i32 = input.trim().parse().expect("Input not an integer");
        if number == target{
            println!("Correct Answer!");
            flag = false;
        }
        else if number > target{
            println!("high!");
        }
        else if number < target{
            println!("less!");
        }
    }
}
