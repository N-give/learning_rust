pub mod rsorts {
    use std::thread;

    pub fn gen_rqsort<T> (arr: &mut[T], rev: bool)
        where T: Eq + Ord + Copy,
    {
        if arr.len() <= 1 {
            return;
        }
        let mut front: usize;
        let mut back: usize;
        let cmp = arr[arr.len() - 1];

        while {
            front = match arr[..(arr.len())]
                .iter()
                .position(|n| {
                    *n > cmp
                })
            {
                Some(i) => i,
                None => arr.len() - 1,
            };

            back = match arr[..(arr.len() - 1)]
                    .iter()
                    .rev()
                    .position(|n| {
                        *n <= cmp
                    })
            {
                Some(i) => arr.len() - 2 - i,
                None => 0,
            };

            front < back
        } {
            // std::mem::swap(&mut arr[front], &mut arr[back]);
            arr.swap(front, back);
        }
        arr.swap(front, arr.len() - 1);
        gen_rqsort(&mut arr[..front], rev);
        gen_rqsort(&mut arr[(front + 1)..], rev);
    }

    // pub fn gen_rqsort<T> (arr: &mut[T], rev: bool)
    pub fn threaded_rqsort (arr: Vec<i32>, rev: bool) -> Result<Vec<i32>, std::io::Error>
    {
        if arr.len() <= 1 {
            // Ok(arr)
            arr.to_vec();
        }
        let mut front: usize;
        let mut back: usize;
        let cmp = arr[arr.len() - 1];

        while {
            front = match arr[..(arr.len())]
                .iter()
                .position(|n| {
                    *n > cmp
                })
            {
                Some(i) => i,
                None => arr.len() - 1,
            };

            back = match arr[..(arr.len() - 1)]
                    .iter()
                    .rev()
                    .position(|n| {
                        *n <= cmp
                    })
            {
                Some(i) => arr.len() - 2 - i,
                None => 0,
            };

            front < back
        } {
            // std::mem::swap(&mut arr[front], &mut arr[back]);
            arr.swap(front, back);
        }
        arr.swap(front, arr.len() - 1);

        let (smaller, greater) = arr.split_at_mut(front);

        let t_smaller = thread::spawn(move || {
            threaded_rqsort(smaller.to_vec(), rev.clone());
        });

        let t_greater = thread::spawn(move || {
            threaded_rqsort(greater.to_vec(), rev.clone());
        });

        let smaller = t_smaller.join().unwrap();
        let greater = t_greater.join().unwrap();

        let mut sorted_arr = vec![];
        sorted_arr.push(smaller);
        sorted_arr.push(greater);

        Ok(sorted_arr)
        // Ok(sorted_arr)
    }
}
