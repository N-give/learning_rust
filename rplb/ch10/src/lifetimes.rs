/*
 *  Every reference in rust has a lifetime
 *  - a special lifetime:
 *      'static: a lifetime that spans the execution of the program
 *      str are always static
 *          ```
 *          let s: &'static str = "I have a static lifetime.";
 *          ```
 *
 */

//lifetime annotations in struct definitions
pub struct ImportantExcerpt<'a> {
    pub part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    pub fn level(&self) -> i32 {
        3
    }

    pub fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

/*
pub fn invalid_lifetime() {
    let r;                  // --------+--'a
    {                       //         |
        let x = 5;          // -+--'b  |
                            //  |      |
        r = &x;             //  |      |
    }                       // -+      |
    println!("{}", r);      //         |
}                           // --------+
*/

pub fn valid_lifetime() {
    let r;                  // --------+--'a
    let x = 5;              // -+--'b  |
    r = &x;                 //  |      |
    println!("r: {}", r);   //  |      |
}                           // -+------+

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len(){
        x
    } else {
        y
    }
}

