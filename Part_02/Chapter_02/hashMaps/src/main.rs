use std::collections::HashMap;

fn main() {
	//Creating HashMaps:
		
	let mut scores = HashMap::new();

	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);// The key is a string for this hashmap

	// Another way to create hash maps is to map two vecs together into a tuple as follows:
	let teams = vec![String::from("Blue"), String::from("Yellow")];
	let start_scores = vec![10, 50];

	let scores: HashMap<_,_> = teams.iter().zip(start_scores.iter()).collect();//We can use _ as rust can infer the type.

	//For types that implement the copy trait like i32 the values are copyed into the hashmap but things that are owned like strings the values are moved into the hashmap (Ownership is taken).


	//Accessing values in a hashmap
	//Using the key we can pass its reference into the get function as follows:
	let score = start_scores.get(&String::from("Blue"));
	//This returns Some(&10);

	//We can also itterate over key value pairs.
	for (key, value) in &start_scores	{
		println!("{} : {}", key, value);
	}

	//Updating HashMaps:
	//If we want to insert a new value and we dont care about the old one we just insert the new value.
	start_scores.insert(String::from("Blue"), 20);
	//If we want to check if a key has a value and if it dose not insert one we use entry
	start_scores.entry(String::from("Yellow")).or_insert(20);

	//Updating values based on the current ones.
	//Insert number of times a word occurs
	let text = "Hello world wonderful world";
	let mut map = HashMap::new();
	for word in text.split_whitespace()	{
		let count = map.entry(word).or_insert(0); //Or insert returns a mut that is count
		*count += 1
	}

	//Hashing Functions

	//By default HashMap uses a cryptographically secure hashing function to prevent Dos attacks however you can also implement your own hashing function
}
