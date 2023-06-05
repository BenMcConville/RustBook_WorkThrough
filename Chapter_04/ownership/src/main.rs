fn main() {
    loop {
	let s = "Hello"; //String comes into scope
	break;
    } // s/String goes out of scope
    let mut s = String::from("Hello"); //Can (does not need to be) be a mutable string
    s.push_str(", World");
    println!("{}", s);
}

fn ownership()	{
    let s1 = give_os();
    let s2 = String::from("Hello");
    let s3 = take_os(s2);
}
fn give_os() -> String	{
    String::from("Hello")
}
fn take_os(a_str: String) -> String	{
    a_str
}
