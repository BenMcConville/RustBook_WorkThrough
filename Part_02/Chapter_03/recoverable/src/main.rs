//Using the result enum ->

/*
enum Result<T, E>	{
	Ok(T),
	Err(E),
}
T & E are generic type paramaters
T- represents the type of value that is returned in a success 
E- represnets the type of error that is returned in a failure

*/

//If we want to see the return type of a module we can give it a type we know its not such as:
// let f: u32 = File::open("file.txt"); the compiler will then tell us the type.

//If we want to do different things based on error types we use the :
use std::io::ErrorKind;

use std::fs::File;

fn main() {
	//If we can open this file we f is an instance of Ok with the file handle otherwise it would have the Err with info about the error.
	let f = File::open("hello.txt");
	let f = match f	{
		Ok(file) => file,
		Err(ref error) if error.kind() == ErrorKind::NotFound => 	{
			match File::create("hello.txt")	{
				Ok(fc) => fc,
				Err(e) => {
					panic!("Tryed to create file but there was a problem: {:?}", error)
				},
			}
		},
		Err(error) => {
			panic!("There was a problem opening the file: {:?}", error)
		}
	};
	let b = File::open("File2.txt").expect("This is the same as unwrap but we pick the output message :)");//unwrap();
}
