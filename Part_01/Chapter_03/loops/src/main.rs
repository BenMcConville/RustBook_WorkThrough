fn main() {
    loop {
    	println!("Hello, world!");
	break;
    } //This will continue untill told to stop (Use break)
    
    let mut num = 3;
    while num != 0 {
	println!("Num: {}", num);
	num = num -1;
    }
    
    let a = [1,2,3,4,5]; // a = (1..4).rev();
    for elem in a.iter()	{
	println!("Elem: {}", elem);
    }
}
