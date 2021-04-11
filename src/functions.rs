pub fn run() {

    greeting("Good Morning", "Elisa");

    let sum = add_two_integers(12, 5);
    println!("Sum is {}", sum);

    // closure

    let add_nums = |n1: i32, n2: i32| n1 + n2; // like an arrow function, can use outside scope vars
    println!("Closure Sum is {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, how are you?", greet, name);
}

fn add_two_integers(n1: i32, n2: i32) -> i32 {
    return n1 + n2;
}