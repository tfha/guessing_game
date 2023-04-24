use std::io; //compiled library from standard library
use rand::Rng;
use std::cmp::Ordering; //the Ordering enum is from cmp in standar library
// Ordering can have the follow states (variants) Less, Greater, Equal
// Result is another enumeration (enum) with the variantes Ok and Err

fn main() {
    println!("Guess the number!"); //println is a macro, not a function

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{ //infinite loop
        println!("Please input your guess.");

        // the function new() is an accosiated function of the string type
        let mut guess: String = String::new(); //mut leads to a mutable parameter
        //NOTE: you don't need to declare type annotation. It pops out automatically

        io::stdin()
            .read_line(&mut guess) // using & indicates a reference variable. Is not stored in memory
            .expect("Failed to read line");
        //if REsult returns Err, the expect message will be printed

        // reuse and shadow the old guess variable. You reuse old stuff too
        //:u32 tells rust to use 32-bit int
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}