// conditionals kinda like you'd expect
// no ternary
// but shorthand if

pub fn run () {
    let age: u8 = 20;
    let check_id: bool = false;

    if check_id && age >= 21 {
        println!("Bartender: what would you like to drink?");
    } else if check_id && age <= 21 {
        println!("Bartender: Sorry you have to leave");
    } else {
        println!("Bartender: can I see your id?");
    }

    let is_of_age = if age >= 21 {true} else {false};

    println!("Is of age: {}", is_of_age);
}
