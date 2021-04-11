// run function that runs everything
pub fn run() {
    println!("Printing!");

    // like formatted strings in JS
    println!("Some number {}", 12); // numbers
    println!("Another number: {}", 14);
    println!("Program says: {}", "Hello World"); // strings
    println!("Floating point: {}", 12.6);
    // println!("{}", true);  booleans do not work
    // println!("{}", false);

    // Each bracket pair (pincer) correlates by postion

    println!("Hi my name is {0} I am from {1}, I like {2}. {0} is {3} years old.", "Rafayel", "Armenia", "Programming", 14);

    // naming prints using pseudo vars

    println!("My name is {name}, I like playing {sport}", name = "Rafayel", sport = "tennis");

    // Traits

    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 7, 8, 6);

    // ^gets the types of placeholders :b :x :o 

    // debug trait

    println!("{:?}", (12, true, "Tuple Element", 12.3)); // :? lists a collection, in this case a tuple

    println!("12 - 4 = {}", 12 - 4);


}