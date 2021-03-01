use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut count = 0u32;

    loop {
        count += 1;
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
       }

       // FOR FUNSIES: count loop and ask the user if they give up. If yes, break
       if count == 10 {
           println!("Do you give up?");

           let mut give_up = String::new();

           io::stdin()
              .read_line(&mut give_up)
              .expect("Failed to read line");


            if give_up.trim() == "yes" {
                println!("Sucker!");
                break
            } else {
                println!("Continuing!");
                continue;
            }

       }

    }
}
