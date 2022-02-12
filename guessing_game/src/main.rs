use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
	println!("Guess the number!");

	let secret_number = rand::thread_rng().gen_range(1..101);
	
	//println!("The secret number is: {}", secret_number);

	loop {
	
		println!("Please input your guess.");

		let mut guess = String::new();
			// mut overrides the default immutable behavior, allowing the variable to be changed (mutable)
			// String is a utf8 string type

		io::stdin()
			.read_line(&mut guess)
				// &mut passes the guess variable by reference
				// passing &guess instead would still pass by reference, but make the reference immutable
			.expect("Failed to read line");
				// defines a default value

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		println!("You guessed: {}", guess);
			 // string interpolation via the printlin! method

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			}
		}

	}

}
