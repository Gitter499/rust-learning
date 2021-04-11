// Vectors -  resizeable arrays

pub fn run() {
    let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut mutable_nums: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("{:?}", mutable_nums);

    // Get val by index

    println!("One value: {}", nums[0]);

    // Reassign values in mutable array

    mutable_nums[4] = 16;
    println!("Mutated Vector: {:?}", mutable_nums);

    // Add to vector

    mutable_nums.push(6);
    mutable_nums.push(8);

    println!("Mutated vector {:?}", mutable_nums);

    // Pop (remove) last value

    mutable_nums.pop();

    // Get length of array

    println!("Vector length: {}", nums.len());

    // Array stack allocation

    println!(
        "Vector uses {} bytes in memory",
        std::mem::size_of_val(&nums)
    ); // Each element occupies 4 bytes (i32)

    // Array slice

    let vector_slice: &[i32] = &mutable_nums[0..2]; // reference an array, can choose what elements to reference using ..

    println!("Slice: {:?}", vector_slice);

    // Loop through Vectors

    for i in mutable_nums.iter() {
        println!("Number: {}", i)
    }

    // Loop and mutate Vectors

    for i in mutable_nums.iter_mut() {
        *i *= 2
    }

    println!("Numbers Vec: {:?}", mutable_nums);
}
