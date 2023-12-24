use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main(){
    println!("Welcome to Guessing Game");
    let secretnumber = rand::thread_rng().gen_range(1..=100);
    println!("secretnumber is: {secretnumber}");
    print!("Please Input the Number: ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to readline");
    println!("You Guessed {guess} ");
    match guess.cmp(&secretnumber) 
    {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
        
    
}