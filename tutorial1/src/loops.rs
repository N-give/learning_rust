pub fn run () {
    let mut cnt = 0;

    // inifinite loop
    loop {
        cnt += 1;
        println!("number: {}", cnt);
        if cnt == 20 { break };
    }

    cnt = 0;
    // while for fizzbuzz
    while cnt <= 100 {
        if cnt % 15 == 0 {
            println!("FizzBuzz");
        } else if cnt % 3 == 0 {
            println!("Fizz");
        } else if cnt % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", cnt);
        }
        cnt += 1;
    }

    for x in 0..100 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }
    }

}
