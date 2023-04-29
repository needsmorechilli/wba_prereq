//types: int (u8, i8, u18, etc), floats, bool, char, tuples and arrays (fixed)
//need to declare types at compilation but compiler can infer

pub fn run (){
    // Default is "i32"
    let x = 1;

    //Default is "f64"
    let y = 2.5;

    //Add explicit type
    let z: i64 = 454545454545;

    //Find max size (calling std library i32)
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i32::MAX);

    // Boolean
    let is_active:bool = true;

    //get boolean from expression
    // char is single quote, can put unicode including emojis

    let is_greeter: bool = 10 < 5;

    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x,y,z, is_active, is_greeter, a1, face));
}