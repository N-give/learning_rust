pub fn run () {
    greeting("Hello", "Nate");
    println!("{}", add(1, 2));

    let get_sum = add(5, 5);
    println!("{}", get_sum);

    // closure - bind functions to variable
    let n3 = 3;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("{}", add_nums(3, 3));

}

fn greeting (greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn add (n1: i32, n2: i32) -> i32 {
    n1 + n2 // no semicolon means return this value
}
