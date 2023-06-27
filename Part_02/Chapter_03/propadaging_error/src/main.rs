use std::{io, Read};
use std::fs::File;

//This function is returning a enum which can be a Ok(Strind) or Err(io::Error). 
//The fact this function can return an error and does not handle it in this function allows the caller to make a more educaded decsion on what to do rather than generalsing a error handling solution for this function.
//This is so common rust has the general syntax for it "?"

fn read_username_from_file() -> Result<String, io::Error>	{
	let f = File::open("Hello.txt");
	
	let mut f = match f	{
		Ok(file) => file,
		Err(e) => return Err(e),
	};
	
	let mut s = String::new();

	match f.read_to_sting(&mut s)	{
		Ok(_) => Ok(s),
		Err(e) => Err(e),	
	}
}

fn ruff() -> Result<String, io::Error>	{ //This code is the same as above
	let mut f = File::open("hello.txt")?; //If error is thrown an error is passed to f if not f is passed the file name from the Ok(file) enum
	let mut s = String::new();
	f.read_to_string(&mut s)?;
	Ok(s)

	//This could be shotened to:
	//File::open("hello.txt")?.read_to_string(&mut s)?;
}
fn main() {
    println!("Hello, world!");
}
