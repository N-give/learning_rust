struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

// essentially the functions to be attached to an "object"
// rust doesn't have objects.
impl Circle {
    pub fn get_x(&self) -> f64 {
        self.x
    }
}

struct Rectangle {
    height: f64,
    width: f64,
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.height * self.width
    }
}

enum Hero {
    Fast,
    Strong(i32),
    Info {name: String, secret: String}
}

fn main() {
    println!("Max {}: {}", "i8", std::i8::MAX);
    println!("Min {}: {}", "i8", std::i8::MIN);
    println!("Max {}: {}", "i16", std::i16::MAX);
    println!("Min {}: {}", "i16", std::i16::MIN);
    println!("Max {}: {}", "i32", std::i32::MAX);
    println!("Min {}: {}", "i32", std::i32::MIN);
    // and so on

    // setting the values for multiple vars
    let (f_name, l_name) = ("Nate", "Givens");

    /*
    * print formatting
    */
    println!("{1}, {0}", f_name, l_name);
    println!("{:.2}", 1.234);
    println!("B: {0:b} H: {0:x} O: {0:o}", 200);

    println!("{ten:>ws$}", ten=10, ws=5);
    println!("{ten:>0ws$}", ten=10, ws=5);

    /*
     * maths
     */
    println!("abs(-4) = {}", (-4i32).abs());
    println!("4^6 = {}", 4i32.pow(6));
    println!("sqrt(9) = {}", 9f64.sqrt());
    println!("cbrt(27) = {}", 27f64.cbrt());
    // .round()
    // .floor()
    // .ceil()
    // .exp()
    // .ln()
    // .log10()
    // f64.to_radians()
    // f64.to_degrees()
    // 4f64.max(5f64) => 5
    // 4f64.min(5f64) => 4
    // 3f64.sin() and other trig functions

    /*
     * Strings
     */
    let some_string = "I'm a string";
    println!("{}", some_string);

    for letter in some_string.chars() {
        match letter {
            'i' => println!("{}", letter),
            _ => println!("Some other letter"),
        }
    }

    /*
     * user input
     */

    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("read in: {}", input);
        } // When using curly braces, no comma is needed after a "case"
        Err(error) => println!("error: {}", error),
    }

    'outer: loop {
        let number: i32 = 10;
        println!("Pick a number!");

        loop {
            let mut line = String::new();
            let input = std::io::stdin().read_line(&mut line);
            let guess: Option<i32> = input.ok().map_or(None, |_| line.trim().parse().ok());

            match guess {
                None => println!("Enter a number"),
                Some(n) if n == number => {
                    println!("You guessed it");
                    break 'outer;
                }
                Some(n) if n < number => {
                    println!("too low");
                }
                Some(n) if n > number => {
                    println!("too high");
                }
                Some(_) => println!("Error"),
            }
        }
    }

    /*
     * interesting things with vectors --> reduce
     */
    println!("sum([1, 2, 3]): {}", sum_vector(&vec![1, 2, 3]));

    /*
     * structs
     */
    let circ1 = Circle {x:10f64, y: 10f64, radius:10f64};
    println!("X: {}, Y: {}, R: {}", circ1.x, circ1.y, circ1.radius);
    println!("X: {}, Y: {}, R: {}", circ1.get_x(), circ1.y, get_radius(&circ1));

    println!("circle area: {}", circ1.area());

    let rect1 = Rectangle{height: 10f64, width: 10f64};
    println!("rectangle area: {}", rect1.area());

    /*
     * enumerated types
     */
    let hulk = Hero::Strong(100);
    let quicksilver = Hero::Fast;
    let spiderman = Hero::Info{
        name: "Spiderman".to_owned(),
        secret: "Peter Parker".to_owned()
    };

    get_info(hulk);
    get_info(quicksilver);
    get_info(spiderman);
}

fn sum_vector(v: &Vec<i32>) -> i32 {
    let sum = v.iter().fold(
        0,
        |mut sum, &x| { sum += x; sum }
    );

    return sum;
}

fn get_radius(circle: &Circle) -> f64 {
    circle.radius
}

fn get_info(h: Hero){
    match h {
        Hero::Fast => println!("Fast"),
        Hero::Strong(i) => println!("lifts {} tons", i),
        Hero::Info {name, secret} => {
            println!("{} is {}", name, secret);
        }
    }
}
