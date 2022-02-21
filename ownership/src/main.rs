fn main() {
	
	string_demo();

	move_demo();

	clone_vrs_copy_demo();

	ownership_transfer_demo();

	tuples_to_control_ownership_demo();

	passing_by_reference_demo();

	mutable_reference_demo();

	dangleing_reference_demo();

}

fn string_demo () {
	let mut s = String::from("Hello"); // String is provisioned on the Heap instead of the stack, as would a string
																		// litteral. This allows it to be mutable.
	s.push_str(", world!"); // appends to mutable string
	println!("{}", s);
} // At the end of the scope, s is dropped from the heap. 

fn move_demo () {

	let _x = 5;
	let _y = _x;
		// This creates a copy of 5 on the stack because it is a primitive.

  let _s1 = String::from("hello");
	let _s2 = _s1;
		// This does NOT create a copy - it moves ownership of the pointer to the string on the heap to _s2.
		// println!("{}", _s1); // This creates a compile error because _s1 has gone out of scope.
	let _s3 = _s2.clone();
		// this DOES crete a deep copy on the heap, allowing _s2 to be accessed.
	println!("{}", _s2);
	println!("{}", _s3);

}

fn clone_vrs_copy_demo() {
	
	let s = String::from("hello"); // s comes into scope

	takes_ownership(s); // Ownership of s is transfered to takes_ownership; s is no longer valid!

	let x = 5;

	makes_copy(x); // x implements copy, so x is still valid after this statement.

}

fn takes_ownership(some_string: String) {
	println!("{}", some_string);
} // at the end of this method, some_string goes out of scope and is removed from the heap.

fn makes_copy(some_primitive: u32) {
  println!("{}", some_primitive);
} // here, only the copy of x called some_primitive goes out of scope


fn ownership_transfer_demo() {

	let s1 = gives_ownership();
	println!("{}", s1);

	let s2 = String::from("This string will be passed into the takes_and_returns method and passed back.");
	let s3 = takes_and_returns(s2);
	println!("{}", s3);
}

fn gives_ownership() -> String {
	let some_string = String::from("this string will be given to the ownership_transfered_demo function");
	some_string
}

fn takes_and_returns( some_string: String) -> String {
	some_string
}

fn tuples_to_control_ownership_demo() {
	let s1 = String::from("hello");
	let (s2, len) = calculate_length(s1); // s1 is transfered to calculate_length, but we can pass it back in a tuple.
	println!("The length of '{}' is {}", s2, len);
}

fn calculate_length(some_string: String) -> (String, usize) {
	let length = some_string.len(); 
	(some_string, length)
}

fn passing_by_reference_demo() {
	let s1 = String::from("hello");

	let len = calculate_length_by_reference(&s1);

	println!("The length of '{}' is {}", s1, len); // because we never passed ownership of s1, it's still available.

}

fn calculate_length_by_reference( some_string: &String ) -> usize {
	// some_string.push_str(", world!"); // This won't work, because you can't modify something you don't own.
	some_string.len()
}

fn mutable_reference_demo() {
	let mut s = String::from("hello");

	let _r1 = &mut s;
	// let r2 = &mut s; // this will fail, because their can only be one mutable reference at a time.
	// println!("{}, {}", r1, r2);
	// further, you can't mix immutable and mutable references
}
	
fn dangleing_reference_demo() {
	//let s1 = danggle();
	let s2 = no_dangle();	
}

// fn dangle() -> &String {
	// let s = String::from("hello");
	// &s // This will create a compile error because s goes out of scope, leaving &s dangling
// }
	

fn no_dangle() -> String {
	let s = String::from("hello");
	s
}
