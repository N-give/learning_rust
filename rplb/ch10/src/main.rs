#[allow(dead_code)]
/*
 *  Generic Types, Traits, and Livetimes
 *  - generics: tools for handling duplication of concepts
 *      - abstract stand-ins for concrete types or other properties
 *  - traits: tells compiler about functionality of a type
 */

mod traits;
mod lifetimes;

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

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

// we can also conditionally implement functions if the inner type T implements other traits
// in this example, we only implement cmp_display if the inner type T implements both Display for
// printing and PartialOrd for comparison
impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("largest number is x = {}", self.x);
        } else {
            println!("largest number is y = {}", self.y);
        }
    }
}

// we can also conditionally implement a trait for ay type that implements another trait
// impl<T: std::fmt::Display> ToString for T {
    // fn
// }

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

    let vals = [2, 3, 1, 6, 34, 6, 0];
    let lval = largest(&vals);
    println!("and the largest is: {}", lval);

    traits::run();

    // lifetimes::invalid_lifetime();
    lifetimes::valid_lifetime();
    let str1 = String::from("abcd");
    let str2 = "xyz";
    let result = lifetimes::longest(str1.as_str(), str2);
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i1 = lifetimes::ImportantExcerpt{ part: first_sentence };
    println!("{}", i1.part);

    let longest2 = longest_with_an_announcement(&str1, str2, "This just in");
    println!("the longest again: {}", longest2);
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

fn largest<T: PartialOrd + Copy>(l: &[T]) -> &T {
    let mut largest = 0;
    for (i, &item) in l.iter().enumerate() {
        if item > l[largest] {
            largest = i;
        }
    }
    &l[largest]
}

// bringing it all together...
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: std::fmt::Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
