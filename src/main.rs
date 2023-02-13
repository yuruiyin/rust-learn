use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::ptr::null;

fn main() {
    // println!("Hello, world!");
    // hello_world();
    // // 打印数字
    // println!("{}", add(2, 5));

    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is {secret_num}");

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}

fn hello_world() {
    println!("hello world1111!")
}

fn add(a: i32, b: i32) -> i32 {
    return a + b
}
