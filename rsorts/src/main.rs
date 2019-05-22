use rsorts::rsorts::qsort;

fn main() {
    let mut a1 = vec![5, 3, 6, 4, 7, 2, 1, 10, 9];
    // println!("{:?}", a1);
    qsort(a1.as_mut_slice(), |a, b| a - b);
    println!("{:?}", a1);
    // test_slice(&a1[1..]);
}
