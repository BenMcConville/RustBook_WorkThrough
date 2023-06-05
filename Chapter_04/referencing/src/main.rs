fn main() {
    println!("Hello, world!");
    without_referencing();
    with_referencing();
    let mut s = String::from("Hello");
    change(&mut s);
}
fn change(s: &mut String)	{
    s.push_str(", World!");
}

fn without_referencing()	{
    let s1 = String::from("Hello");
    let (len, s1) = find_len(s1);
    println!("{} has length {}", s1, len);
}
fn find_len(a_str: String) -> (usize, String)	{
    (a_str.len(), a_str)
}

fn with_referencing()	{
    let s1 = String::from("Hello");
    let len = find_len_r(&s1);
    println!("{} has length {}", s1, len);
}
fn find_len_r(s: &String) -> usize	{
    s.len()
}
