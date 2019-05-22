#[allow(unused_variables)]
pub mod rsorts {
    pub fn qsort<F>(arr: &mut [i32], cmp: F)
    where
        F: Fn(&i32, &i32) -> i32 + Copy,
    {
        if arr.len() <= 1 {
            return;
        }
        let mut front = 0 as usize;
        let mut back = (arr.len() - 1) as usize;
        // println!("length: {}", back);
        println!("i:\n{:?}", arr);
        while front < back {
            front = match arr[front..(back + 1)]
                .iter()
                .position(|n| cmp(n, &arr[arr.len() - 1]).is_positive())
            {
                Some(i) => i,
                None => front + 1,
            };

            back = match arr[front..(back + 1)]
                .iter()
                .rev()
                .position(|n| cmp(n, &arr[arr.len() - 1]).is_negative())
            {
                Some(i) => i,
                None => front,
            };
            println!("front: {}, back: {}", front, back);

            let tmp = arr[back];
            arr[back] = arr[front];
            arr[front] = tmp;
        }
        let tmp = arr[front];
        arr[front] = arr[arr.len() - 1];
        arr[arr.len() - 1] = tmp;
        println!("e:\n{:?}\n", arr);
        qsort(&mut arr[..front], cmp);
        qsort(&mut arr[front..], cmp);
    }
}
