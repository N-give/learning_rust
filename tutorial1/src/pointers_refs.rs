pub fn run () {
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("{:?}", (arr1, arr2));

    // with non-primatives, like vectors, if you asign another variable to a piece of data, the
    // first variable will no longer hold that value. You'll have to use a reference (&) to point
    // to the resource

    let v1 = vec![4, 5, 6];
    let v2 = &v1;

    println!("{:?}", (&v1, v2));
}
