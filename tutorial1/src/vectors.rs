// vectors are resizeable arrays - linked lists?

pub fn run () {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", numbers);
    println!("first element: {}", numbers[0]);

    let mut mut_num: Vec<i32> = vec![0, 1, 2, 3, 4];
    println!("{:?}", mut_num);
    mut_num[0] = 4;
    println!("{:?}", mut_num);
    println!("array length: {}", mut_num.len());

    println!("array takes: {}", std::mem::size_of_val(&mut_num));

    // slice array
    let slice: &[i32] = &numbers[0..2]; // start position and non-inclusive end
    println!("Slice: {:?}", slice);

    mut_num.push(6);
    println!("{:?}", mut_num);
    mut_num.pop();
    println!("{:?}", mut_num);

    // for x in numbers.iter() {
    //     println!("{}", x);
    // }

    for x in mut_num.iter_mut() {
        *x *= 2;
        println!("{}", x);
    }

    println!("{:?}", mut_num);
}
