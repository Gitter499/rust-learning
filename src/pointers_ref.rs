// Ref pointers - point to a resource in memory

pub fn run() {
    // Primitive Array

    let array_1 = [1,2,3];

    let array_2 = array_1;

    // Non primitives will lose their data when reassigned

    // Vectors are not primitives

    let vec_1 = vec![1,2,3];

    let vec_2 = &vec_1;


    println!("Values: {:?}", (array_1, array_2)); // array_1 wont lose its data
    println!("Values: {:?}", (&vec_1, vec_2));
}