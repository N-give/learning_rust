// structs used to create custom data types


struct Color {
    red: u8,
    green: u8,
    blue: u8
}


struct Color2 (u8, u8, u8);

struct Person {
    first: String,
    last: String
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first: first.to_string(),
            last: last.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first, self.last)
    }

    fn set_last(&mut self, last: &str) {
        self.last = last.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first, self.last)
    }
}

pub fn run () {
    let mut c1 = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    println!("red: {}, green: {}, blue: {}", c1.red, c1.green, c1.blue);

    c1.blue = 200;
    println!("red: {}, green: {}, blue: {}", c1.red, c1.green, c1.blue);

    let mut c2 = Color2(255, 0, 0);
    println!("red: {}, green: {}, blue: {}", c2.0, c2.1, c2.2);
    c2.1 = 150;
    println!("red: {}, green: {}, blue: {}", c2.0, c2.1, c2.2);

    let mut p1 = Person::new("Nate", "Givens");
    println!("first: {}, last: {}", p1.first, p1.last);
    println!("peron: {}", p1.full_name());

    p1.set_last("the Great");
    println!("peron: {}", p1.full_name());
    println!("peron: {:?}", p1.to_tuple());
}
