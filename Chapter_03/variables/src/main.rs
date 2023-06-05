fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    
    let y: f32 = 3.0; //f32
    let y = 3.0; //f64

    let b: bool = false; //With explicit type annotation
    let b = true;
    
    let c = 'z';
    let c = ' ';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;  //This process is called destructing
    println!("The value of y: {}", y);
    let index_Zero = tup.0; //Another way of indexing tuples

    let arry = [1,2,3,4,5];
    //Arrays indexed like:
    let first = arry[0];

    another_function();
}

fn another_function()	{
    println!("Another Function");
}
