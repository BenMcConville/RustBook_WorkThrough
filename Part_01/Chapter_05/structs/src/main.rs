struct User	{
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,// Fields
}

let user_01 = User	{ //Instance of User
	email: String::from("someone@example.com"),
	username: String::from("someusername123"),
	sign_in_count: 5,
	active: true,
};

fn main() {
    println!("User 1 email: {}", user_01.email);
}

fn build_user(email: String, username: String) -> User	{
	User {
		email: , //email, these are optional
		username: , //username,
		sign_in_count: 1,
		active: true,
	}
}

//If we wanted to create a new user from a previous user we could do it two ways:

/*
let user_02 = User	{
	sign_in_count: 0,
	active: true,
	email: user_01.email,
	username: user_01.username,
} 
*/
let user_02 = User	{
	sign_in_count: 0;
	active: true,
	..user_01,//struct update syntax
}

 
