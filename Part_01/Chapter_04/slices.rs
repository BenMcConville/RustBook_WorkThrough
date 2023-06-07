
fn main()	{
    let s = String::from("Hello, World");
    let hello = &s[0..5]; // eqv to &s[..5]
    let word = first_word(&s);

}

fn first_word(s: &str) -> &str	{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate()	{
 	if item == b' '	{
		return &s[..i]
	}
    }
    &s[..]
}
