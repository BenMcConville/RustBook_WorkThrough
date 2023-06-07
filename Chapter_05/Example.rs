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
	let rec_02 = Rec { length: 40, width: 10 };
	let _rec_03 = Rec { length: 45, width: 60 };
	println!("Rect_01 is {:#?}", rec_01); // can use {:?}
	//println!("Area of Rec is {}", area(&rec_01)); This used with func
	println!("Rec 01 holds Rec 02 {}", rec_01.can_hold(&rec_02));
}
/* We can always create functions but to limit which types can call these functions we implement methods
fn area(rectangle: &Rec) -> u32	{
	rectangle.length * rectangle.width
}*/ 
impl Rec	{
	fn area(&self) -> u32	{
		self.length * self.width
	}
	fn can_hold(&self, r: &Rec) -> bool	{
		self.area() > r.area()
	}
}
