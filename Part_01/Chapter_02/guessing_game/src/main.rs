extern crate rand;
use std::io;	//Use io libary for standard io commands
		//io from std libary
use rand::Rng;

use std::cmp::Ordering;

fn main() {
    println!("Guess Number");
    
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("Secret Number is: {}", secret_number);    

    loop	{
    println!("Input Guess");
    
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
	.expect("Failed to read line");
    
    //let guess: u32 = guess.trim().parse()
	//.expect("Please type a number"); original code but crashes when incorrect input is given

    let guess: u32 = match guess.trim().parse()	{
	Ok(num) => num,
	Err(_) => continue,
    }; // pattern matching

    println!("You guessed: {}", guess);	
    
    match guess.cmp(&secret_number)	{	//cmp method called on any values that can be compaired (This compairs guess to sn)
        Ordering::Less	=> println!("Too Small"), //These are called arms(match expression and code that should be run)
        Ordering::Greater => println!("Too Big"),
        Ordering::Equal => {
		println!("You Win");
		break;
        }
    }
    }
}
