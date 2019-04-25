// 2 types:
// - immutable fixed-length string
// - String growable heap-allocated data structure

pub fn run () {
    let mut hello = String::from("Hello");

    let hello_length = hello.len();

    println!("{}, len: {}", hello, hello_length);

    hello.push(' ');
    hello.push('W');
    hello.push_str("orld!");

    println!("{}", hello);

    println!("Is empty: {}", hello.is_empty());
    println!("Is empty: {}", hello.contains("World"));
    println!("Is empty: {}", hello.replace("World", "Nate"));

    // loop through string
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut cap_string = String::with_capacity(10);

    cap_string.push('a');
    cap_string.push('b');

    println!("{}", cap_string);
    // println!("{}", cap_string);

    assert_eq!(10, cap_string.capacity());
}
