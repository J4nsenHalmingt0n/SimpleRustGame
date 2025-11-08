use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess A Number !");
    let mut rng = rand::thread_rng();
    let x: u8 = rng.gen_range(1..10);
    let y: u8 = rng.gen_range(1..10);
    let result_number = x + y;
    println!("x + {y} = {result_number} ?");
    let message = String::from("Try to input a number");
    println!("{}", message);
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed Input");
    let guess: u8 = guess.trim().parse().expect(&message);

    println!("You Guess : {guess}");

    match guess.cmp(&x) {
        Ordering::Less => println!("It Too less"),
        Ordering::Greater => println!("It Too Big"),
        Ordering::Equal => println!("You Right that is was {x}"),
    }
}
