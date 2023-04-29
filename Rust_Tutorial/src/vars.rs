//variables live in block and are immutable

pub fn run(){
    let name = "Brad";
    let mut age = 37;
    println!("My name is {} and I am {}", name, age);
    age = 38;
    println!("My name is {} and I am {}", name, age);

    // Define constant (need type)
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assign multiple vars
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}