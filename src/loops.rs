pub fn run() {
    let mut count = 0;

    // normal loop

    loop {
        count += 1;
        println!("Number {}", count);

        if count == 20 {
            break;
        }
    }

    // while loop

    // FizzBuzz!

    // let mut num = 0;
    // while num <= 100 {
    //     if num % 15 == 0 {
    //         println!("FizzBuzz");
    //     } else if num % 3 == 0 {
    //         println!("Fizz");
    //     } else if num % 5 == 0 {
    //         println!("Buzz");
    //     } else {
    //         println!("{}", num)
    //     }
    //     num += 1;
    // }

    // For range

    for i in 1..100 {
        if i % 15 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i)
        }
    }
}
