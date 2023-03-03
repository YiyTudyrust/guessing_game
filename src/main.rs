use std::io; // это библиотека для ввода и вывода данных
use rand::Rng; // эта библиотека для генерации чисел от 1 до 100
use std::cmp::Ordering; // это библиотека для сравнение цифр

fn main () {
    let name = String::from("Guessing game");
    let secret_number:u8 = rand::thread_rng().gen_range(1..=100); // генерируется число от 1 до 100

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