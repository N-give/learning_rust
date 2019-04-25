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
}
