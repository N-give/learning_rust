/* ownership rules:
 * 1. each value in Rust has a variable that's its /owner/
 * 2. there can only be one owner at a time
 * 3. when the owner goes out of scope, the value will be dropped
 */
// scope: essentially, a block of code

fn main() {
    {
        // s is invalid because it hasn't been declared yet
        // let s = "hello"; // now that s have been declared, it is available to use
        // s is still available
    } // this is where the scop for s ends
      // s is no longer available, this is outside of its scope
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);

    let mut s2 = s;
    s = s2;
    s2 = s.clone();

    println!("{}", s);
    println!("{}", s2);

    // Rust copies values that can be easily stored on the stack so .clone() is not necessary in
    // cases similar to this
    // others:
    // - all integer types
    // - bool
    // - all float types
    // - char
    // - Tuples if they only contain copy types

    let x = 5;
    let y = x;
    let z = true;
    println!("x: {}, y: {}, z: {}", x, y, z);

    takes_ownership(s);

    makes_copy(x);

    let mut some_string2 = gives_ownership();

    println!("{}", some_string2);

    some_string2 = takes_and_gives_back(some_string2);

    println!("{}", some_string2);

    // passing the string ownership to the function relenquishes ownership from the calling
    // function and the string will be dropped at the end of the called function unless
    // ownership is passed back
    let some_str3 = String::from("testing3");
    println!("{}", some_str3);

    let (some_str3, some_str3_len) = calc_len1(some_str3);
    println!("{}: length of string: {}", some_str3, some_str3_len);

    // ownership can, however, be borrowed for a time and given back without having to return the
    // value from the called function and the data isn't dropped because the called function never
    // had ownership over the data. it was only borrowed
    let str4 = String::from("test4");
    // &: reference operator
    let str4_len = calc_len2(&str4);
    println!("str4: {}, length: {}", str4, str4_len);

    // This is called passing a mutable reference
    // only 1 mutable reference to a value can be loaned at a time
    let mut str5 = String::from("hello");
    println!("str5: {}", str5);
    change(&mut str5);
    println!("str5: {}", str5);

    // a there can be multiple non-mutable references loaned for a value
    // but no mutable reference may be loaned out while any non-mutable
    // references are loaned

    /*
     * Slice type
     */
    let str6 = String::from("This is a string");
    let fw = first_word(&str6);
    println!("{}", fw);

    let str7 = String::from("This is another string");
    let this = &str7[0..4]; // equivalent to &str[..4]
    let is = &str7[5..8]; // if you want the last byte in the string can omit the
                          // last index &str[4..]
                          // can also take a slice of the whole string &str[..]
    println!("{} {}", this, is);

    let str7_first = first_word2(&str7);
    println!("{}", str7_first);

    // this syntax for slices will also work for arrays
}

fn takes_ownership(some_string: String) {
    // some string comes into scope
    println!("{}", some_string);
} // some_string goes out of scope and 'drop' is called
  // this frees the backing memory -> the variable is no longer available in the calling function

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // here, some_integer goes out of scope and nothing special happens because it was a copy

fn gives_ownership() -> String {
    // moves its return value into the function that calls it
    let some_string = String::from("hello"); // some_string comes into scope
    some_string // some string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// it is possible to return multiple values using a tuple
fn calc_len1(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a string
    (s, length)
}

// it is also possible to borrow a value so it does not have to pass ownership back to the calling
// function
// passing a value in this manner doesn't allow any modification of the data
fn calc_len2(s: &String) -> usize {
    s.len()
}

// this will allow us to borrow a value and modify it without having to return the ownership to the
// calling function
fn change(str: &mut String) {
    str.push_str(", world");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
