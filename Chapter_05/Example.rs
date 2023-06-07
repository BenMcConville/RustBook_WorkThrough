// This program will calculate the area of a rectangle
//To print struct we need to add debugging information

#[derive(Debug)]
struct Rec	{
	length: u32,
	width: u32,
}
fn main()	{
	let rec_01 = Rec{
		length: 50,
		width:	20
	};
	println!("Rect_01 is {:#?}", rec_01); // can use {:?}
	println!("Area of Rec is {}", area(&rec_01));
}
/* We can always create functions but to limit which types can call these functions we implement methods
fn area(rectangle: &Rec) -> u32	{
	rectangle.length * rectangle.width
}*/ 
impl Rec	{
	fn area(&self) -> u32	{
		self.length * self.width
	}
}
