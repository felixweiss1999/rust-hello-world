use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess!");
    let mut guess = String::new();
    let bytes: usize = io::stdin()
        .read_line(&mut guess)
            .expect("Failed to read line");
    println!("You guessed: {}, and entered {bytes} bytes.", guess);
}