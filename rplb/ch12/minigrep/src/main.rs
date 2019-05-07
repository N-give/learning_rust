use std::env;

fn main() {
    // std::env::args will panic! on any invalid unicode
    // if invalide unicode is required, use std::env::args_os

    // .collect() requires that the type be specified because it can format the
    // return value in many different ways
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
