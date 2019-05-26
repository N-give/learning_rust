extern crate crossbeam_epoch as epoch;

use epoch::Atomic;
use rsorts::rsorts::gen_rqsort;
extern crate rand;
use std::fs;
use std::io;
use std::io::prelude::*;
// use std::io::BufReader;

fn main() -> io::Result<()> {
    let fp = fs::File::open("nums.txt")?;
    let fp = io::BufReader::new(fp);
    // let mut nums: Vec<i32> = Vec::new::with_capacity(500);
    let mut nums: Atomic<Vec<i32>> = Atomic::new(Vec::with_capacity(500));

    for line in fp.lines() {
        let n = line
            .unwrap()
            .trim()
            .parse::<i32>()
            .expect("Failed to parse integer");

        nums.push(n);
    }
    let sorted_arr = gen_rqsort(nums, false)?;
    println!("{:?}", sorted_arr);
    // println!("properly sorted: {}", nums.is_sorted());

    Ok(())
}
