//If we want to create a structure without field names we can create tuple structs

fn main()	{
	struct Color(i32, i32, i32);
	struct Point(i32, i32, i32);


	let black = Color(0, 0, 0);
	let origin = Point(0, 0, 0);


}
