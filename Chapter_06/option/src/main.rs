/*enum Option<T>	{ //Option<T> is so useful it is already loaded in the prelude. (Its wittten now to show inner workings)
	Some(T),
	None,
}*/


//As Some and None variant are so common they do not need to be preloaded with the enum like Option::Some() or Option::None, they can just be called.

fn main() {
	let some_number = Some(5);
	let some_string = Some("A string");

	let absent_number: Option<i32> = None;
}
