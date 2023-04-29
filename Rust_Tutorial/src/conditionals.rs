//Check condition and act on result aka if/else

pub fn run(){
    let age: u8 = 21;
    let check_id: bool = true;
    let knows_person_of_age = true;
    // if/else
    if age >=21 && check_id || knows_person_of_age{
        println!("Bartender: What would you like to drink?");
    } else if age<21 && check_id {
        println!("Bartender: Sorry you have to leave");
    } else {
        println!("Bartender: I'll need to see your ID");
    }

    //shorthand if (no ternary in rust)
    let is_of_age = if age>= 21 {true} else {false};
    println!("Is Of Age : {}", is_of_age);

}