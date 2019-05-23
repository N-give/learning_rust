use rsorts::rsorts::rqsort;

fn main() {
    // let mut a1 = vec![1, 3, 2];
    let mut a1 = vec![1, 8, 6, 4, 7, 2, 9, 10, 5];
    //  test_pos(&a1);
    // println!("{:?}", a1);
    rqsort(a1.as_mut_slice(), false);
    println!("{:?}", a1);
    // test_slice(&a1[1..]);
}
