pub fn run() {
    let name = "Rafayel"; // immutable by default
    let mut age = 14;
    println!("My name is {0}, I am {1} years old", name, age);
    age = 15;
    println!("A year has passed!");
    println!("My name is {0}, I am {1} years old", name, age);

    // constants

    const ID: i32 = 700; // type required

    println!("My ID is {}", ID);

    // Multiple vars (possibly called a tuple assingment, have to read Rust book lol)

    let (friend_1, friend_2, friend_3) = ("Micheal", "Matt", "John");

    println!("My friends are: {0}, {1}, and {2}", friend_1, friend_2, friend_3);




}