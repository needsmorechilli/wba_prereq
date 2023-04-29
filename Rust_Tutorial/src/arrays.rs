//arrays  is fixed list with elements of same data types
//format: [type;length]
 
use std::mem;

pub fn run(){
    let mut numbers: [i32; 4] = [1,2,3,4];

    // Re-assign values
    numbers[2] = 20;

    println!("{:?}", numbers);

    // Get single value
    println!("Single Value: {}", numbers[0]);

    // Get array length
    println!("Array Length: {}", numbers.len());

    //Arrays are stack allocated, std called on top
    println!("Array occupies {} byes", mem::size_of_val(&numbers));

    //Get slice
    let slice: &[i32] = &numbers[0..2];

    println!("Slice: {:?}", slice); //use ? inside as is array
}