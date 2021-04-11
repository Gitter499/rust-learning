pub fn run() {
    // types and values
    let person: (&str, &str, i8) = ("Rafayel", "Armenia", 14);

    println!("{0} is from {1} and is {2} years old", person.0, person.1, person.2)
}