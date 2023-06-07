//Two types of ip addresses v4 and v6:
enum IpAddrKind	{ //Used over stucts when different types exist (this enum has its own custom data type)
	V4,
	V6,//V4 and V6 are variants of the enum
}

struct IpAddr	{
	kind: IpAddrKind,
	address: String,
}

enum IpAddr01	{ //Attaching data directly to enum
	V4(String),
	V6(String),
}
enum IpAddr02	{// Advantage of Enum over struct
	V4(u8, u8, u8, u8),
	V6(String),
}
fn route(ip_type: IpAddrKind)	{}
fn main() {
	let home = IpAddr	{
		kind: IpAddrKind::V4,
		address: String::from("127.0.0.1"),
	};
	let home01 = IpAddr01::V4(String::from("127.0.0.1"));
	let home02 = IpAddr02::V4(127,0,0,1);

	let loopback = IpAddr	{
		kind: IpAddrKind::V6,
		address: String::from("::1"),
	};
	let loopback01 = IpAddr01::V6(String::from("::1"));
	let loopback02 = IpAddr02::V6(String::from("::1"));

	let four = IpAddrKind::V4;
	let six = IpAddrKind::V6;
	route(IpAddrKind::V4);
	route(IpAddrKind::V6);
}
