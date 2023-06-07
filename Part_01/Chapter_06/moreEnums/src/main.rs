enum Message	{
	Quit,				//No associated data
	Move { x: i32, y: i32 },	//Struct (Currently not defined)
	Write(String),			//String
	ChangeColor(i32, i32, i32),	//3 i32s
}
//Creating an enum like this is similar to creating a struct except all the variants are grouped together under the Message type.

/*
impl Message	{
	fn call(&self)	{
		// Code executes with reference to &self
	}
}
*/


fn main() {
    println!("Hello, world!");
}
