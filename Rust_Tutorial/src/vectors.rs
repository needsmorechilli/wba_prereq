//arrays  is resizable arrays
//format: Vec<type> = vec! initialized array
 
use std::mem;

pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4];

    // Re-assign values
    numbers[2] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value
    numbers.pop();

    println!("{:?}", numbers);

    // Get single value
    println!("Single Value: {}", numbers[0]);

    // Get vector length
    println!("Vector Length: {}", numbers.len());

    //vector are stack allocated, std called on top
    println!("Vector occupies {} byes", mem::size_of_val(&numbers));

    //Get slice
    let slice: &[i32] = &numbers[0..2];

    println!("Slice: {:?}", slice); //use ? inside as is array

    // Loop through vector values
    for x in numbers.iter(){
        println!("Numbers: {}", x);
    }

    //Loop and mutate values
    for x in numbers.iter_mut(){
        *x *=2;
    }

    println!("Numbers Vec: {:?}", numbers);
}