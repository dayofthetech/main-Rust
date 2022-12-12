//this is to use the less, equal wording at the match line
use std::cmp::Ordering;
use std::io;
//importing random crate - Rng is a trait
//typing cargo doc --open will build a doc specific to the dependensies being used, nice
use rand::Rng;

// 12/12 - Guessing game where user enter a number and game will mention over or under
//      extra features can include
//      [] A message to alert the user to enter int only
//      [] A x amount of trys, countdown
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");

    //loop infite times - Im assuming better than an if statemnt
    loop {
        println!("Please input your guess.");

        //here the type is a string
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //but we change mutate the type since we assigned a mut into the guess var above
        //we grab that guess and turn it into a unassignaded 32

        // OG - Below, doing it this way affects the game as it does not respond when user enters
        //  something else that is not a number
        // let guess: u32 = guess.trim().parse().expect("Please type a number");

        // Update - we added the match expression since now the game will be looking for
        //  an int. Is the job of parse() to turn the input into a number and to return a value
        //Ok checks to see if its a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        //the cmp method compares two values, in this case guess.compare to the secret num
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                // By adding the curly brackets you turn the statement into an expression
                println!("You win!");
                break;
            }
        }
    }

}
