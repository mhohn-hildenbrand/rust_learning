fn main() {

	// let x = 5;
		// This won't compile, because we later assign x = 6, but by default x is immutable. We need to explicitly declare x
		// as mutable.
	let mut x = 5;
	println!("The value of x is: {}", x);

	x = 6;
	println!("The value of x is: {}", x);

	const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;
		// constants are ALWAYS immutable, and you must specify a type. You can't calculate a constant at run time, it must
		// be set to a constant expression.

	println!("Three hours in seconds is: {}", THREE_HOURS_IN_SECONDS);

	let y = 5;

	let y = y + 1;
		// This is "Shadowing" y; Note that shadowing in the same context results in a new y when used later.

	{
		let y = y * 2;
		println!("The value of y in the inner scope is: {}", y)
			// Note that shadowing in a context leaves the original alone.
	}

	println!("The value of y is: {}", y)

}
