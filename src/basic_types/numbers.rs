
//Integer

pub fn run1() {
    let x = 5u32;
    assert_eq!("u32".to_string(), type_of(&x).to_string());

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

pub  fn  run2() {
    assert_eq!(i8::MAX, 127 ); 
    assert_eq!(u8::MAX, 255); 

    println!("Success!");
}

pub fn run3() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; // 1024 + 255 + 63 + 255 = 1597
    assert_eq!(v, 1597);
    println!("Success {v}!");
}

// Floatting-point numbers

// Fill the blank to make it work
pub fn run4(){
    let x = 1_000.000_1; // ?
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64
    println!("x: {}, y: {}, z: {}", x, y, z);
    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success !");
}

//Make it work in two distinct ways
pub fn run5() {
    assert!(0.1_f32 + 0.2_f32 == 0.3_f32);
    println!("Success!");
}

// Range
pub fn run6() {
   let mut sum = 0;
    for i in -3..2 {    // -3, -2, -1, 0, 1 sum = 
        sum += i
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}",c);
    }
}

use std::{ops::{Range, RangeInclusive}};
pub fn run7() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}

pub fn run8(){
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1u8.wrapping_sub(2) == 255); // underflow, wraps around
    assert!(3 * 50 == 150);
    assert!(((9.6f64 / 3.2f64) * 10.0).round() / 10.0 == 3.0);
    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
