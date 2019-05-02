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

    /*
     * Strings:
     * Strings are utf-8 encoded and can handle many different chars
     * Strings do not support indexing: s[0] will produce an error
     */

    // creates new empty string
    let mut s1 = String::new();
    let sample = "initial contents";
    // this creates a String from a str literal
    let s2 = sample.to_string();
    // it can work directly on str literals too
    let s3 = "inital content".to_string();
    let s4 = String::from("intial content"); // s3 and s4 are equivalent computationally
    let s5 = String::from("ሴ "); // chars that aren't just english
    println!("{}", s5);

    // Strings can be mutated, unlike strs
    let mut s6 = String::from("Hello");
    println!("{}", s6);
    s6.push(',');
    s6.push_str(" world");
    println!("{}", s6);
    s6.push_str(&s2);
    println!("{}", s6);

    // string contatenation
    // when doing this, only the first item can be a String, and the rest must be &str
    // this uses the add method -> fn add(self, s: &str) -> String
    // this method consumes the first String
    // if we don't want s2 to lose ownership, we should set the concatenation equal to itself
    // s2 = s2 + ...
    let s7 = s2 + " " + &s4;
    println!("{}", s7);

    // a less wild and more readable way to contatenate multiple strings
    let s8 = format!("{}-{}-{}", s1, s3, s4);
    // and this method doesn't consume the strings
    println!("{}", s3);
    println!("{}", s8);
    println!("{}", s8.len());

    // slicing strs
    let s9 = "hello";
    println!("{}", &s9[..3]);

    for c in s9.chars() {
        println!("{}", c);
    }

    for c in s9.bytes() {
        println!("{}", c);
    }

    /*
     * Hash maps
     * HashMap is least often used from collections, so it is not automatically
     * brought into scope
     *
     * all keys of a hash map must be of the same type and all values must be
     * of the same type
     */
    let mut scores1 = std::collections::HashMap::new();
    scores1.insert(String::from("Blue"), 10);
    scores1.insert(String::from("Yellow"), 50);

    // a hash map can also be created from a vector of tuples
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let init_scores = vec![10, 50];
    let scores2: std::collections::HashMap<_, _> = teams.iter().zip(init_scores.iter()).collect();

    // hash maps will copy simple types like i32... but not Strings
    // Strings will be consumed
    let field_name = String::from("favorite color");
    let field_val = String::from("blue");
    let mut map1 = std::collections::HashMap::new();
    map1.insert(field_name, field_val);
    // field name and field value are no longer available

    let team_name = String::from("Blue");
    let sc = scores2.get(&team_name);
    match sc {
        Some(&val) => println!("{} score: {}", team_name, val),
        None => println!("Not in map"),
    }
    for (k, v) in &scores1 {
        println!("{}: {}", k, v);
    }

    // this implementation of hash map only allows one value for each key
    let mut map2 = std::collections::HashMap::new();
    // this will override whatever value is currently in the map
    map2.insert(String::from("blue"), 10);
    map2.insert(String::from("blue"), 25);
    match map2.get(&String::from("blue")) {
        Some(&val) => println!("{} score: {}", "blue", val),
        None => println!("Not in map"),
    }

    // only inserting a value if the key doens't already have a value
    let mut map3 = std::collections::HashMap::new();
    map3.insert(String::from("blue"), 10);
    map3.entry(String::from("yellow")).or_insert(55);
    map3.entry(String::from("blue")).or_insert(35);
    println!("{:?}", map3);

    // update value if key already exists based on current value
    let text = "hello world said the world";
    let mut map4 = std::collections::HashMap::new();
    for word in text.split_whitespace() {
        let count = map4.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map4);
}
