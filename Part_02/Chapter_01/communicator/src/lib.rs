/*
src/lib.rs has this module hierarchy:

communicator
	client
	network
		server
	test

----------------- After seperating all the modules we have:

communicator
	src
		client.rs
		lib.rs
		network
			mod.rs
			server.rs
		test
*/
pub mod client; //By adding a semi-colon we are telling rust to look for an external file.
pub mod network;



// This is the tests module:
//-Here exist a test mod which contains the function it_works
// As the name sugests test is used for testing

#[cfg(test)]
mod tests	{
	
	use super::client;

	#[test]
	fn it_works()	{
		//client::connect();
		//To move back one layer we can either call ::client::connect() (start from root directory);		or call super::client::connect(); Finally we can also use "use super::client" to call our funtion as: 
		client::connect();
	}
}
