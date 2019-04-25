pub fn run() {
    // defaults to i32
    let x = 1;

    // defaults to f64
    let y = 2.5;

    // explicit typing
    let z: i64 = 478612348643;

    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    let is_active = true;

    let is_greater: bool = 10 < 5;

    let a1 = 'a';
    let uc = '\u{1f600}'; // unicode characters

    println!("{:?}", (x, y, z, is_active, is_greater, a1, uc));
}
