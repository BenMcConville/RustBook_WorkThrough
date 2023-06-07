fn plus_one(x: Option<i32>) -> Option<i32>	{
	match x	{
		None => None,
		Some(i) => Some(i + 1),//i matching to a value is called binding to the value that is contained in Some.
	}
}

fn main()	{
	let five = Some(5);
	let six = plus_one(five);
	let none = plus_one(None);
	//As enums are stored on the stack we can call five again:
	let six_02 = plus_one(five);

	let some_u8 = 2u8;
	print_number(some_u8);
}

// The place holder
fn print_number(n: u8)	{
	match n {
		1 => println!("One"),
		2 => println!("Two"),
		3 => println!("Three"),
		4 => println!("Four"),
		5 => println!("Five"),	
		6 => println!("Six"),
		7 => println!("Seven"),
		_ => (), //The place holder (catch all)
	}
}
