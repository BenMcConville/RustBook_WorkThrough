/*
src/lib.rs has this module hierarchy:

communicator
	client
	network
		server

----------------- After seperating all the modules we have:

communicator
	src
		client.rs
		lib.rs
		network
			mod.rs
			server.rs
*/
pub mod client; //By adding a semi-colon we are telling rust to look for an external file.
pub mod network;

#[cfg(test)]
mod tests	{
	#[test]
	fn it_works()	{
	
	}
}
