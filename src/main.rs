use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("secret number: {}",secret_number);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess:u32 = match guess.trim().parse(){
            Ok(num)=>{
                num
            },
            Err(_)=>{
                println!("your input must be number");
                continue
            }
        };

        println!("you guess: {}",guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("to small"),
            Ordering::Greater => println!("To big"),
            Ordering::Equal => {
                println!("you win");
                break
            }
        }
    }
}
