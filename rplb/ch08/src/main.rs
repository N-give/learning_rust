#[allow(dead_code)]
#[allow(unused_variables)]
/*
 * Rust's standard library offers many useful data structures
 * a few more notable ones are:
 *   - vector: can store a variable number of values next to each other
 *   - string: a collection of chars
 *   - hash map: can associate a value with a particular key
 *     - a particular implementation of a more general data structure called map
 *   - There are more that can be found in the documentation
 */

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    /*
     * Vectors:
     * - type: Vec<T> where T is the type of data that the vector will store
     */

    // the type of values to be stored in the vectore must be declared if the
    // variable is declared in this fashion
    let v1: Vec<i32> = Vec::new();

    // Here we don't need to tell the compiler what the type is because the
    // macro handles that for us
    let v2 = vec![1, 2, 3];

    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    println!("{:?}", v3);

    let third: &i32 = &v3[2];
    // this can't be here because an element of v3 is current being borrowed
    // immutably
    // v3.push(9);
    println!("{}", third);

    // but this is okay because third goes out of scope
    v3.push(9);

    // the get method will either return the value or return None if the index
    // is outside of the vector so the match construct allows us to safely
    // handle either case
    match v3.get(2) {
        // the & is safer incase the element is not a primative type and it's
        // not just copied
        Some(&third1) => println!("the third element is {}", third1),
        None => println!("There is no third element"),
    };

    println!("{}", v3[2]);

    match v3.get(7) {
        Some(third) => println!("the eighth element is {}", third),
        None => println!("There is no eighth element"),
    };

    // of course vectors can be incremented over
    let v4 = vec![100, 32, 57];
    for i in &v4 {
        println!("{}", i);
    }

    let mut v5 = vec![100, 32, 57];
    for i in &mut v5 {
        // this is a dereference just like in C
        // the only difference is that i must have permission to mutate the
        // variable
        *i += 50;
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        match i {
            SpreadsheetCell::Int(n) => println!("int: {}", n),
            SpreadsheetCell::Float(f) => println!("float: {}", f),
            SpreadsheetCell::Text(s) => println!("text: {}", s),
        }
    }
}
