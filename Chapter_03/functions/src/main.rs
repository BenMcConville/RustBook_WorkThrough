fn main() {
    println!("Hello, world!");
    another_function(5, 2);
}

/*------------ Functions ------------*/

fn another_function(x: i32, y: i32)	{ //Define param for passing
    println!("Another Function with arguments: {} and {}", x, y);
}

fn expression_vs_statement()	{
    let y = { // The code inside is an expression
	let x = 3;
	x + 2
    };
}

fn return_functions(x: i32) -> i32	{
    x + 5 
}

