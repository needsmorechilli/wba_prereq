//primitive str = immutable fixed length in memory
//String = Growable, heap data struct


pub fn run(){
    let mut hello = String::from("Hello ");
    
    //Get length using len method
    println!("Length: {}", hello.len());

    //push is for char type
    hello.push('W');

    //push string only works for String
    hello.push_str("orld!");

    // Capactiy in bytes
    println!("Capactiy: {}", hello.capacity());

    //Check if empty
    println!("Is Emtpy: {}", hello.is_empty());

    // Contains
    println!("Contains 'World' {}", hello.contains("World!"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    //Assertion testing, only show error if failed
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}",s);
}