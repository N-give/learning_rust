#![allow(dead_code)]
/*
 * Packages Crates and Modules
 *
 * - Packages: Cargo feature that allows to build, test, and share crates
 *      - a package can contain zero or one library crates and as many binary crates
 *        as you want
 *      - There must be at least one crate in a package
 * - Crates: tree of modules that produce a library of executable
 * - Modules: 'use' keyword let you control the scope and privacy of paths
 * - A path: way of naming an item such as a struct, function, or module
 *      - use: can be used to bing paths into scope
 *      - when using use with functions, it is idiomatic to only specify the
 *        name of the parent so as to express that the function was not defined
 *        locally
 *      - It is also important to remember (the compiler won't let you forget)
 *        not to bring two things with the same name into the same scope
 *          - an example would be std::io::Result and std::fmt::Result
 */

mod sound;

mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

mod menu {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

mod performance_group {
    // this allows other scopes that bring this module in to use the instrument
    // module as well
    pub use crate::sound::instrument;
    pub fn clarinet_trio() {
        instrument::woodwind::clarinet();
        instrument::woodwind::clarinet();
        instrument::woodwind::clarinet();
    }
}

// this is not considered idiomatic
// use std::collections::HashMap;
// while this is:
use std::collections;

// types brought into scope with the same name can be renamed with 'as'
use rand::Rng;
use std::fmt::Result;
use std::io::Result as IoResult;

// multiple crates can be brought in in one line
use std::{cmp::Ordering, io::BufRead};
// this can also be done when one path extends another
use std::io::{self, Write};
// there is also a glob operator for these sorts of things
// use std::*

fn main() {
    // absolute path
    println!("{}", crate::sound::instrument::woodwind::clarinet());
    // relative path
    println!("{}", sound::instrument::voice::vocals());

    let mut v1 = plant::Vegetable::new("squash");
    v1.name = String::from("butternut squash");
    println!("{} are okay.", v1.name);

    // This won't compile because Vegetable::id is not public
    // println!("{} ID is {}", v1.name, v1.id);

    let o1 = menu::Appetizer::Soup;
    let o2 = menu::Appetizer::Salad;
    {
        // let's bring Salad into the path so we don't have to keep retyping the
        // whole path for each of the 1 other times that we use it here
        // Here is how we bring it into scope with a relativ path
        use self::menu::Appetizer::Soup;
        use menu::Appetizer::Salad;
        let o3 = Salad;
        let o4 = Soup;
        use_enum(&o3);
        use_enum(&o4);
    }

    use_enum(&o1);
    use_enum(&o2);
    use_enum(&o1);

    performance_group::clarinet_trio();
    performance_group::instrument::voice::vocals();

    let mut map = collections::HashMap::new();
    map.insert(1, 2);
    println!("{:?}", map);

    let sec_num = rand::thread_rng().gen_range(1, 101);
    println!("{}", sec_num);
}

fn use_enum(app: &menu::Appetizer) {
    match app {
        menu::Appetizer::Soup => println!("Soup"),
        menu::Appetizer::Salad => println!("Salad"),
    };
}

// fn func1() -> Result {
//fn body
// }

// fn func2() -> IoResult<()> {
// fn body
// }
