#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("called");
        // match self {
        //     Quit => println!("Quit"),
        //     Move => println!("Move"),
        //     Write => println!("Write"),
        //     ChangeColor => println!("ChangeColor"),
        // }
    }
}

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);
    // route(four);
    // route(six);
    println!("home: {:#?}", home);
    println!("home: {:#?}", loopback);

    let mut str1 = String::from("a");
    let len1 = borrow(&str1);
    println!("length: {}", len1);
    mut_borrow(&mut str1);
    let len2 = borrow(&str1);
    println!("length: {}", len2);
}

// fn route(ip_type: IpAddrKind) {
// do something
// println!("{:?}", ip_type);
// }

fn borrow(s: &String) -> usize {
    s.len()
}

fn mut_borrow(s: &mut String) {
    s.push('b');
}
