// Arrays in Rust - fixed length ordered list with same data types

// use std::mem to simplify std::mem to just mem
pub fn run() {
    // use semicolon to denote type and length
    let nums: [i32; 5] = [1, 2, 3, 4, 5];
    let mut mutable_nums: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", nums);

    // Get val by index

    println!("One value: {}", nums[0]);

    // Reassign values in mutable array

    mutable_nums[4] = 16;
    println!("Mutated array: {:?}", mutable_nums);

    // Get length of array

    println!("Array length: {}", nums.len());

    // Array stack allocation

    println!(
        "Array uses {} bytes in memory",
        std::mem::size_of_val(&nums)
    ); // Each element occupies 4 bytes (i32)

    // Array slice

    let array_slice: &[i32] = &mutable_nums[0..2]; // reference an array, can choose what elements to reference using ..

    println!(" Slice: {:?}", array_slice);
}
