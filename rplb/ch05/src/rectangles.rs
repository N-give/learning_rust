#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// attaches the methods specific to a struct to instances of those structs
// essecitally the second half of classes in C++ from C
// impl -> implementation
// dereferencing is implicit with methods in rust
// no -> needed like in C/C++
// p1.distance(&p2) --> (&p1).distance(&p2)
impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// there can also be multiple impl blocks for a single struct
impl Rectangle {
    // associated functions are functions associated with a struct that don't take &self as an
    // argument
    // they are often used as "constructors" for the struct because of this
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub fn rectangles() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 40,
        height: 40,
    };

    // the :: operator is used to access methods of a struct
    let sq1 = Rectangle::square(10);

    println!("rectangle 1: {:?}", rect1);
    // the # specifies pretty printing for the struct
    println!("rectangle 1: {:#?}", rect1);
    println!("The area of the rectange is {} square pixels", rect1.area());

    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3: {}", rect1.can_hold(&rect3));

    println!("sq1: {:#?}", sq1);
}
