fn main() {
	let number = 3;

	if number < 5 {
		println!("Condition was true");
	} else {
		println!("Condition was false");
	}

	if number % 5 == 0 {
		println!("Number is divisible by 5");
	} else if number % 3 == 0 {
		println!("Number is divisible by 3");
	} else if number % 2 == 0 {
		println!("Number is divisible by 2");
	} else {
		println!("Number does not have factors 5, 3, or 2");
	}

	let _result = if number == 5 { 0 } else { 1 }; // if is an expression!

	let mut count: i8 = 0;

	let _result = 'loop_label_one: loop { //loops are expressions, with end value passed to break
		println!("count: {}", count);
		let mut remaining: i8 = 10;
		loop {
			println!("remaining: {}", remaining);
			if remaining == 9 {
				break;
			} else if count == 2 {
				break 'loop_label_one count; // the result of the loop
			}
			remaining -= 1;
		}
		count += 1;	
	};

	println!("loop output: {}", _result );

	let mut number = 3;

	while number != 0 {
		println!("while number: {}", number);
		number -= 1;
	}

	let items = [ 10, 20, 30, 40, 50 ];

	for element in items {
		println!("for item: {}", element);
	}

}
