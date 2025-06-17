pub fn run1(){
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2]; // -> &[i32] is a slice type

    let s2: &str = "hello, world";

    println!("Success!");
}

pub fn run2(){
     let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];
    
    // Modify '8' to make it work
    // TIPS: slice( reference ) IS NOT an array, if it is an array, then assert! will be passed: Each of the two chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
    assert!(std::mem::size_of_val(&slice) == 16);

    println!("Success!");
}

pub fn run3(){
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Fill the blanks to make the code work
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);

    println!("Success!");
}

pub fn run4(){
    let s = "你好，世界";
    // Modify this line to make the code work
    let slice = &s[0..3]; // 

    assert!(slice == "你");

    println!("Success!");
}