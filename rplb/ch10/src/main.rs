#[allow(dead_code)]
/*
 *  Generic Types, Traits, and Livetimes
 *  - generics: tools for handling duplication of concepts
 *      - abstract stand-ins for concrete types or other properties
 *  - traits: tells compiler about functionality of a type
 */

mod traits;

#[derive(Debug)]
struct Point1<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

// this is a generic impl, but we could specify for types
// ex. impl Point1<i32> {...}
impl<T> Point1<T> {
    fn getx(&self) -> &T {
        &self.x
    }
    fn gety(&self) -> &T {
        &self.y
    }
}

impl Point1<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p_int = Point1 {x: 5, y: 10};
    let p_float = Point1 {x: 1f64, y: 4.5f64};

    let p_mix = Point2 {x: 5i32, y: String::from("Hello world")};

    println!("{}", p_int.getx());
    println!("{}", p_float.gety());
    println!("{}", p_float.distance_from_origin());
    println!("{:?}", p_mix);

    traits::run();
}

// the logic in these two functions is identicle... lets abstract it so it doesn't depend on type
// and need to be rewritten by hand multiple times
fn largest_i32(l: &[i32]) -> i32 {
    let mut largest = l[0];
    for &num in l.iter() {
        if num > largest {
            largest = num;
        }
    }
    largest
}

fn largest_char(l: &[char]) -> char {
    let mut largest = l[0];
    for &c in l.iter() {
        if c > largest {
            largest = c;
        }
    }
    largest
}

// this won't compile because we need to apply a trait to our generic types. more on that later
/*
fn largest<T>(l: &[T]) -> T {
    let mut largest = l[0];
    for &item in l.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
*/
fn largest<T: PartialOrd + Copy>(l: &[T]) -> T {
    let mut largest = l[0];
    for &item in l.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
