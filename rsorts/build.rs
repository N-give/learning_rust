use rand::prelude::*;
use rand;
use std::fs;
use std::io::Write;

fn main() -> Result<(), std::io::Error> {
    let mut rng = rand::thread_rng();

    let mut num: Vec<i32> = (1..500).collect();
    num.shuffle(&mut rng);

    let mut fp = fs::File::create("nums.txt").expect("failed to open file");
    for n in num.into_iter() {
        writeln!(&mut fp, "{}", n).expect("failed to write number");
    }
    Ok(())
}
