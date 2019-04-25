// tuples group together values of different types
// max 12 elements

pub fn run () {
    let person: (&str, &str, i8) = ("Nate", "Indiana", 25);
    println!("{} is from {}, and he is {}", person.0, person.1, person.2);
}
