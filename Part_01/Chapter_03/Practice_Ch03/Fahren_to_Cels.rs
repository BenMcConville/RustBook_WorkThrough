use std::io;

fn main()	{
	loop {
		println!("1) F->C , 2) C->F");
		let mut selection = String::new();
		io::stdin().read_line(&mut selection)
			.expect("Failed to read line");
		
		let selection: u8 = match selection.trim().parse()	{
			Ok(num) => num,
			Err(_) => continue
		};
		println!("Enter Temp");
		let mut temp = String::new();
		io::stdin().read_line(&mut temp)
			.expect("Failed to read line");
		let mut temp: f32 = match temp.trim().parse()	{
			Ok(num) => num,
			Err(_) => continue
		};
		if selection == 1	{
			temp = (temp - 32.0) / 1.8;
			println!("Temp in C is {}", temp);
		} else if selection == 2	{
			temp = (temp * 1.8) + 32.0;
			println!("Temp in F is {}", temp);
		} else {
			println!("Non valid selection picked");
		}
	}
}
