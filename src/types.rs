/*
   Basic Types

   Integers (u is unsigned, i is signed, the corresponding number is how many bits they take in):
   u8, i8, u16, i16, u32, i32, u64, i64, u128, i128

   Unsigned can't have negatives

   Floating point numbers: f32, f64

   Boolean: bool

   Chars: char

   Tuples

   Arrays
*/

pub fn run() {
    // Rust is saying to use snake case but I ain't some Python noob smh
    #![allow(non_snake_case)]

    // inferred to i32
    let inferredNum = 1;

    // inferred to f64
    let inferredFloat = 12.5;

    // explicit typings

    let bigNum: i64 = 66165166515484789;
    let smallNum: i8 = 12;
    let nonNegativeNum: u32 = 17;
    let negativeNum: i32 = -19;
    let decimalNumSmall: f32 = 16.43;
    let decimalNumBig: f64 = 16.4858395830;

    println!("Max i32 num: {}", std::i32::MAX); // 2147483647

    // Bools

    let isHuman: bool = true;
    let isAlien: bool = false;

    let isBigger: bool = 14 > 7;

    // use single quutoes for chars

    let c1 = '🎇';


    println!(
        "{:?}",
        (
            bigNum,
            smallNum,
            nonNegativeNum,
            negativeNum,
            decimalNumBig,
            decimalNumSmall,
            isHuman,
            isAlien,
            inferredFloat,
            inferredNum,
            isBigger,
            c1
        )
    );
}
