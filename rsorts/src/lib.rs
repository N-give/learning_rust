pub mod rsorts {
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
                    n > &cmp
                })
            {
                Some(i) => i,
                None => arr.len() - 1,
            };

            back = match arr[..(arr.len() - 1)]
                    .iter()
                    .rev()
                    .position(|n| {
                        n <= &cmp
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
}
