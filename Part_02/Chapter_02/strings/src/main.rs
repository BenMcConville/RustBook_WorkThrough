fn main() {
	//Creating new string
	let mut s = String::new();

	//To convert a string literal to a String:
	let s = "This is a String literal".to_string();
	// We can also use:
	let s = String::from("New string literal");

	
	//Updating Strings:
	s.push_str(" here");
	//If we want to add a reference to a string we use:
	let mut s1 = String::from("Hello, ");
	let s2 = String::from("World!");
	s1.push_str(&s2);

	//We can also use:
	let s3 = s1 + &s2 //s1 goes out of scope here

	//For complicated string formatting we can use the above concatination or:
	let s1 = String::from("tic");
	let s2 = String::from("tac");
	let s2 = String::from("toe");
	let s = format!("{}-{}-{}",s1,s2,s3);


	//Indexing Strings:
	let len = String::from("Hola").len(); //This looks at how many bytes the Vec is storing
	let len = String::from("    ").len(); //You may say this is 4 but its 8 because rust because this is the number of bytes it takes to encode this.

	//If you need to reference a string use slices or use .char() or .bytes() to sepertate out the string.	
}
