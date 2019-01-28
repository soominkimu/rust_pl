use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    //println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();  // :: associated function (static method)

        io::stdin().read_line(&mut guess)  // mutable
            .expect("Failed to read line");  // io::Result enumerations

        let guess: u32 = match guess.trim().parse() { // shadow the previous value
            Ok(num) => num,
            Err(_)  => continue,
        };

        println!("You guessed: {}", guess);  // {} placeholder

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("Too win!");
                break;
            }
        }
    }
}
