use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read line");
    buf
}

fn main() {
    let secret_num: u8 = rand::rng().random_range(1..=100);
    loop {
        println!("Enter your guess: ");
        let num: u8 = match input().trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input!");
                continue;
            }
        };
        
        match num.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}