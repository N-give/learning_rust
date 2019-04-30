mod option_match;
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

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // Won't be able to compile because y is of type Option<i8> instead of i8
    // will be able to use a match structure to get the i8 out of the option
    // let sum = x + y;
    // println!("{}", sum);
    let mut sum = option_add(x, y);
    match sum {
        None => println!("None was returned"),
        Some(i) => println!("sum = {}", i),
    };

    sum = option_add(12, None);
    match sum {
        Some(i) => println!("Sum = {}", i),
        _ => println!("No cases were matched, so we fell to the default case"), // _ is a placeholder that will catch any other cases
                                                                                // that were not explicitly handled
                                                                                // if we don't want to do anything here we can use "_ => (),"
    };

    // like the match construct we can use if let to extract the value in an Option if there is
    // only one case that we really care about
    // this allows us to ignore all other cases that are not important at this time without writing
    // all match cases
    sum = option_add(12, y);
    if let Some(num) = sum {
        println!("using 'if let'");
        println!("sum = {}", num);
    }

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

    option_match::test_coin();
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

fn option_add(x: i8, y: Option<i8>) -> Option<i8> {
    match y {
        None => None,
        Some(i) => Some(i + x),
    }
}
