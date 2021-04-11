// str = immutable fixed-length string
// String = Grows, heap-allocation data structure nonsense, basically mutable strings

pub fn run() {
    // gonna use camel case cause too lazy to supress warnings

    let fixed_string = "Hello";
    let mut better_string = String::from("Hello ");

    // Get length (works for both)

    println!("String length: {}", fixed_string.len());
    println!("String length: {}", better_string.len());

    // String manipulation

    // better_string.push('W'); push a char

    better_string.push_str("Dumbass"); // push a string

    // Find capacity in bytes

    println!("Capacity: {}", better_string.capacity());

    // Find if empty
    println!("Empty? {}", better_string.is_empty());

    // Contains

    println!("Contains Dumbass? {}", better_string.contains("Dumbass"));

    // Replace

    println!("Replace: {}", better_string.replace("Dumbass", "World"));

    // String looping

    for word in better_string.split_whitespace() {
        println!("{}", word);
    }

    // Create a string with length capacity

    let mut mutable_string = String::with_capacity(20);

    mutable_string.push('a');
    mutable_string.push('b');
    // push by char

    // Assertions

    // assert_eq!(2, mutable_string.len()); true assertion, no fail
    // assert_eq!(3, mutable_string.len()); failed assertion

    assert_eq!(20, mutable_string.capacity()); // same thing here

    println!("{}", mutable_string);

    println!("{:?}", (fixed_string, better_string));
}
