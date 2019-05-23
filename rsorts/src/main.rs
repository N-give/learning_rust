use rsorts::rsorts::gen_rqsort;
extern crate rand;
use rand::Rng;
// use std::io::BufReader;

fn main() {
    // let mut a1 = vec![1, 3, 2];
    // let mut a1 = vec![1, 8, 6, 4, 7, 2, 9, 10, 5, 12, 60, 34, 76, 8, 5, 23, 5, 4,33, 2, 6, 56, 7, 8, 54, 34, 65, 7, 8, 56, 3, 4, 5, 7, 5, 6, 345];
    //  test_pos(&a1);
    // println!("{:?}", a1);
    let mut nums = [0i8; 32];
    rand::thread_rng().fill(&mut nums);
    gen_rqsort(&mut nums, false);
    println!("{:#?}", nums);
    // test_slice(&a1[1..]);
}
