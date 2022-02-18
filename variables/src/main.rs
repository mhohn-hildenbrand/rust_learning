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

	println!("The value of y is: {}", y);

	// Scalar Types - single values

	// * Integer Types, signed and unsigned

	let _myu8: u8 = 255;	
		// the underscore supresses the warning that the unused variable is intentional?
	let _myi8: i8 = -128;	
	let _myu16: u16 = 65535;	
	let _myi16: i16 = -32728;	
	let _myu32: u32 = 4294967295;	
	let _myi32: i32 = -2147483648;	
	let _myu64: u64 = 18446744073709551615;	
	let _myi64: i64 = -9223372036854775808;
	let _myu128: u128 = 340282366920938463463374607431768211455;	
	let _myi128: i128 = -170141183460469231731687303715884105728;	
	let _myusize: usize = 18446744073709551615; // I'm working on a 64 bit machine
	let _myisize: isize = -1;

	//* Other forms of literals

	let _my_decimal: u16 = 1_000; // Same as 1000
	let _my_hex = 0xfe;
	let _my_octal = 0o75;
	let _my_binary = 0b1111_0000;
	let _my_byte = b'A'; //only works in u8?

	//* floating point tyoes

	let _my_f64: f64 = 6.4;
	let _my_f32: f32 = 3.2;

	//* basic arithmetic

	let _my_sum = 5 + 10;
	let _my_difference = 95.5 - 4.3;
	let _my_product = 4*30;
	let _my_quotient = 56.7 / 32.2;
	let _my_floored = 2/3; //results in 0 because the type is evaluated as int
	let _my_remainder = 43 % 5;

	//* Booleans

	let _my_bool: bool = true;

	//* Character

	let _my_char: char = 'z'; // stores a unicode character

	// Compound Types

	// * Tuples

	let _my_tuple: (i32, f64, char) = (500, 6.4, 'A');
	let (_my_x, _my_y, _my_z) = _my_tuple;
	println!("Using Tuple destructuring: {}", _my_y);
	println!("Using Tuple indexing: {}", _my_tuple.1);

	let _my_unit_value: () = (); //special type "unit type" that only has one value, (). this is the default return type of
					// expressions that don't have explicit returns.

	// * Arrays

	let _my_array: [i32; 5] = [1, 2, 3, 4, 5];
		// Allocates memory on the stack rather than the heap.
	let _my_initialized_array = [ 7, 5 ]; // initialized an array of length 5 with value 7 in each place 

	println!("Using Array indexing: {}", _my_array[2]);
		// Note that using an out-of-bounds index will throw a runtime error

}
