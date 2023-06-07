fn main()	{
	let num = 15;
	let mut fib01: u32 = 1;
	let mut fib02: u32 = 0;
	for _i in 1..(num-1)	{
		let temp = fib01 + fib02;
		fib02 = fib01;
		fib01 = temp;
	}
	println!("{} th fib number is {}", num, fib01);
}
