// arrays: fixed list with elements of same data type

pub fn run () {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);
    println!("first element: {}", numbers[0]);

    let mut mut_num: [i32; 5] = [0, 1, 2, 3, 4];
    println!("{:?}", mut_num);
    mut_num[0] = 4;
    println!("{:?}", mut_num);
    println!("array length: {}", mut_num.len());

    println!("array takes: {}", std::mem::size_of_val(&mut_num));

    // slice array
    let slice: &[i32] = &numbers[0..2]; // start position and non-inclusive end
    println!("Slice: {:?}", slice);
}
