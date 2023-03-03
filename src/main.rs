use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main () {
    let name = String::from("Guessing game");
    let secret_number:u8 = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Hay, welcome to {name}. This is guessing game.");
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess:u8 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>{
                println!("Please enter a number from 1 to 100");
                continue;
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small."),
            Ordering::Equal => {println!("You win!");break;},
            Ordering::Greater => println!("Too big."),

        }
    }
}