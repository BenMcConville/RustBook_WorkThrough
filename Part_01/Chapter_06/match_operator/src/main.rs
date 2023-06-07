// From 1999-2008 the us minted ONLY quarters with different designs from each state.

#[derive(Debug)]
enum UsState	{
	Alabama,
	Alaska,
	//etc...
}

enum Coin	{
	Penny,
	Nickel,
	Dime,
	Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> i32	{
	match coin	{	//match can pattern match on many cagegories but in this case matches based on Coin variants.
		Coin::Penny => {
			println!("It's a Penny!!");
			1
		},
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter(state) => {//Example of extracting <T>
			println!("State Quarter From: {:?}", state);
			25
		},
	}
}
fn main() {
    println!("Hello, world!");
}
