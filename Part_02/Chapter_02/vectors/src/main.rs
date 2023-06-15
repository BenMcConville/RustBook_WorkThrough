fn main() {
	//Creating empty vector of type i32:
	let v: Vec<i32> = Vec::new();   


	//Creating non-empty vector with type i32:
	let mut v = vec![1, 2, 3]; //vec! infers type i32.

	// To add values to a vector we call push:
	v.push(4);
	v.push(5);
	v.push(6);

	// Getting values from a vec

	let third: &i32 = &v[2];

	// Or
	
	let third: Option<&i32> = v.get(2);

	println!("Hello, world!");
}

enum SpreadsheetCell	{
	Int(i32),
	Float(f64),
	Text(String),
}

fn vec_with_different_types()	{
	let row = vec![
		SpreadsheetCell::Int(3),
		SpreadsheetCell::Float(3.2),
		SpreadsheetCell::Text(String::from("Here")),
	];
} 
