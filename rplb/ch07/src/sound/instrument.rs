pub mod woodwind {
    pub fn clarinet() -> String {
        // super moves up a code block from the current scope similar to
        // ../ in bash, you need a super for each scope you wish to move
        // up
        println!("{}", super::breathe_in());
        String::from("clarinet")
    }
}
pub mod voice {
    pub fn vocals() -> String {
        println!("{}", super::breathe_in());
        String::from("vocals")
    }
}
pub mod strings {
    pub fn guitar() -> String {
        String::from("guitar")
    }
}

fn breathe_in() -> String {
    String::from("take a breath")
}
