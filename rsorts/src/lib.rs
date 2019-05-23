pub mod rsorts {
    use std::thread;

    pub fn rqsort<'a>(arr: &'a mut [i32], rev: bool) {
        if arr.len() <= 1 {
            return;
        }
        let mut front: usize;
        let mut back: usize;
        let cmp = arr[arr.len() - 1];
        println!("begin:\n{:?}", arr);
        println!("cmp: {}", cmp);

        while {
            front = match arr[..(arr.len())]
                .iter()
                .position(|n| {
                    println!("{}", n);
                    n > &cmp
                })
            {
                Some(i) => i,
                None => arr.len() - 1,
            };
            println!("front: {}, value: {}", front, arr[front]);

            back = match arr[..(arr.len() - 1)]
                    .iter()
                    .rev()
                    .position(|n| {
                        println!("{}", n);
                        n <= &cmp
                    })
            {
                Some(i) => arr.len() - 2 - i,
                None => 0,
            };

            println!("back: {}, value: {}", back, arr[back]);
            front < back
        } {
            let tmp = arr[front];
            arr[front] = arr[back];
            arr[back] = tmp;
            println!("intermediate: {:?}", arr);
        }
        let tmp = arr[front];
        arr[front] = arr[arr.len() - 1];
        arr[arr.len() - 1] = tmp;
        // println!("end:\n{:?}", arr);

        let t1 = thread::spawn(|| {rqsort(&mut arr[..front], rev)});
        let t2 = thread::spawn(|| {rqsort(&mut arr[(front+1)..], rev)});

        t1.join().unwrap();
        t2.join().unwrap();
        // rqsort(&mut arr[..front], false);
        // rqsort(&mut arr[(front+1)..], false);
    }
}
