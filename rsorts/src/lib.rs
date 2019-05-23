pub mod rsorts {
    pub fn rqsort(arr: &mut [i32], rev: bool) {
        if arr.len() == 1 {
            return;
        }
        let mut front = 0usize;
        let mut back = (arr.len() - 2) as usize;
        let cmp = arr[arr.len() - 1];
        println!("begin:\n{:?}", arr);
        println!("cmp: {}", cmp);

        while front < back {
            front = arr[..(arr.len())]
                .iter()
                .position(|n| {
                    println!("{}", n);
                    n > &cmp
                })
                .unwrap();
            println!("front: {}, value: {}", front, arr[front]);

            back = arr.len()
                - 2
                - arr[..(arr.len() - 1)]
                    .iter()
                    .rev()
                    .position(|n| {
                        println!("{}", n);
                        n <= &cmp
                    })
                    .unwrap();
            println!("back: {}, value: {}", back, arr[back]);

            let tmp = arr[front];
            arr[front] = arr[back];
            arr[back] = tmp;
        }
        let tmp = arr[front];
        arr[front] = arr[arr.len() - 1];
        arr[arr.len() - 1] = tmp;
        // println!("end:\n{:?}", arr);
    }
}
