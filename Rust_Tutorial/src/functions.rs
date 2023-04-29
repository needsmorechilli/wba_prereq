//store blocks of code for reuse

pub fn run(){
    greetings("Hello", "Jane");

    //Bind function values to variables
    let get_sum = add(5,5);
    println!("Sum: {}", get_sum);

    //Closure, can use outside variables 
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2:i32| n1 + n2 + n3;
    println!("C Sum: {}", add_nums(3,3));
}

fn greetings(greet:&str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

//arrow to represent return, no semi-colon means its the return
fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}