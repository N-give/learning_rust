/*  some command line arguments to cargo test
 *  to send flags to the binary separate from cargo test with --
 *  - flags to the binary
 *      --test-threads = 1: sets number of parallel threads to 1
 *      --nocapture: let's printing from the program
 *  - flags to carto test
 *      --test <test file name>
 *
 *  running specific test functions
 *  cargo test <test name>: will run test matching test name
 *  cargo test <partial name>: will run tests containing partial match
 *
 *  typically 2 types of tests
 *  - unit tests: small and more focused, testing one module in isolation at a
 *  time - can test private interfaces
 *  - integration tests: are entirely external to library and use code in the
 *  same way any other external code would, using only the public interface and
 *  potentially exercising multiple modules per test
 *
 *  - tests can be grouped into separate files based on the functionality they are testing
 *  - ./tests/common/mod.rs allows a more modular way to create functions shared
 *  between test files
 *      - files in subdirectories of the tests directory don't get compiled as
 *      separate crates or have sections in the test output
 */

#[cfg(test)]
mod tests {
    // this brings all things in the outer scope into this scope
    use super::*;

    // this type of error checking function cannot be used with #[should_panic]
    #[test]
    // ignores this test function unless --ignored: runs only ignored tests
    #[ignore]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // #[test] tells the compiler that these methods are tests
    // They will be run during debugging, but won't be compiled in production
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let name = "Carol";
        let result = greeting(name);
        assert!(
            result.contains(name),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    // using should panic to test that the function handles errors as expected
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}
