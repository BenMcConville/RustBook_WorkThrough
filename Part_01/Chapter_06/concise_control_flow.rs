//Consider looking foronly one pattern match and ignoring the rest we could use match as follows however it is very vorbose:

fn main()	{
	let n = Some(0u8);
	match n {	//This is very longwinded
		Some(3) => println!("three"),
		_ => (),
	}
	//We could use "if let" instead
	if let Some(3) = n	{
		println!("Three");
	} else	{
		println!("Not three");
	}// we can also include an else for the _ case

}
