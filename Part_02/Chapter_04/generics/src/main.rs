//Using generic type on structs:
//We can also define multiple generics using <T, U>

struct Point<T> {
    x: T,
    y: T,
}
//Using generics in methods
impl<T> Point<T>    {
    fn x(&self) -> &T   {
        &self.x
}
}
fn setPoints()  {
    let p1 = Point{x: 1, y: 2};
    let p2 = Point{x: 1.0, y: 2.0};
}


fn main() {
    println!("Hello, world!");

    let nums1 = vec![1,2,3,4,5];
    let nums2 = vec![5,6,7,8,9];

    //Instead of rewriting the code we can create a function

    let max_num1 = findMax(&nums1);
    let max_num2 = findMax(&nums2);

    //Genrics work in a similar way, we identify a repatition and can use abstraction to create a function.
    // An example is how do we find the largest in a list of i32s and chars without repeating code?

    
}
//This code is repeated even though it does the same thing.
fn findMaxChar(list: &[char]) -> char   {
    let mut largest = list[0];
    
    for &elem in list.iter() {
        if elem > largest   {
            largest = elem;
        }
    }

    println!("The largest number is: {}", largest);
    largest
}

fn findMax(list: &[i32]) -> i32    {
    let mut largest = list[0];
    
    for &elem in list.iter() {
        if elem > largest   {
            largest = elem;
        }
    }

    println!("The largest number is: {}", largest);
    largest
}


//Creating generic largest function
//As not all types can be ordered whe need to use a trait.
fn largest<T>(list: &[T]) -> T  {
    let mut largest = list[0];
    
    for &elem in list.iter() {
        if elem > largest   {
            largest = elem;
        }
    }

    println!("The largest number is: {}", largest);
    largest
}
