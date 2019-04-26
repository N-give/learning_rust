mod rectangles;
// using structs

// These must use owned strings (not &str) to ensure the lifetime of the variables
struct User {
    username: String,
    email: String,
    sign_in_cnt: u64,
    active: bool,
}

// Tuple struct without named fields
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let u1 = build_user(String::from("user1@example.com"), String::from("user1"));

    println!(
        "username: {}, email: {}, sign_in_cnt: {}, active: {}",
        u1.username, u1.email, u1.sign_in_cnt, u1.active,
    );

    let u2 = User {
        email: String::from("user2@example.com"),
        username: String::from("user2"),
        ..u1 // no comma
    };
    println!(
        "username: {}, email: {}, sign_in_cnt: {}, active: {}",
        u2.username, u2.email, u2.sign_in_cnt, u2.active,
    );

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!(
        "Color: red: {}, green {}, blue: {}",
        black.0, black.1, black.2
    );

    println!("Point: x: {}, y: {}, z: {}", origin.0, origin.1, origin.2);

    rectangles::rectangles();
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_cnt: 1,
    }
}
