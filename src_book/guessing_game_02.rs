use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // Opening Bracket

    let apples:i32 = 5; // immutable
    let bananas:i32 =6;
    let mut count:i32 = 0;
    println!("apples = {} and bananas ={}", apples, bananas);

    println!("Guess A Number. ");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("This your guess number  {}", count);
        println!("Please input your guess...  ");
        let mut guess = String::new();
        count += 1;


        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");


        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
            };

        println!("You guessed: {}", guess);
        // println!("The secret number is: {}", secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Win !");
                break;
                }
            }
        }

    // Closing Bracket
}
