mod outermost {
	pub fn middle_function()	{}
	
	fn middle_secret_function()	{}
	
	mod inside	{
		pub fn inner_function()	{}
		
		fn secret_function()	{}
	}
}

pub mod a	{
	pub mod series	{
		pub mod of	{
			pub fn nested_modules()	{

			}
		}
	}
}

use a::series::of; //This syntax does not bring the children into scope

fn func_call()	{
	of::nested_modules();
	//a::series::of::nested_modules(); //This can get quite long when calling this function alot so we can ruduce it by calling use a::series::of;
}

//As enums also have a namespace they can be called with use

enum TrafficLight	{
	Red,
	Yellow,
	Green,
}

use TrafficLight::{Red, Yellow}; //Multiple modules can be called with {}
// To import all the modules we can use {*}

fn callingEnum()	{
	let red = Red;
	let yellow = Yellow;
	let green = TrafficLight::Green;
}

//As outermost is a private mod only mods on the same level or lower can access it so our root module try_me can access it. As inside is private only modules on the same or lower layers can access it.

fn try_me()	{
	outermost::middle_function(); //Only valid function call.
	//outermost::middle_secret_function();
	//outermost::inside::inner_function();
	//outermost::inside::secret_function();
}
