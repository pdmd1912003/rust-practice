pub fn run1() {
    let n = 4;
    match_number(n);
}
fn match_number(n: i32) {
    match n {
        // Match a single value
        1 => println!("One!"),
        // Fill in the blank with `|`, DON'T use `..` or `..=`
        2|3|4|5 => println!("match 2 -> 5"),
        // Match an inclusive range
        6..=10 => {
            println!("match 6 -> 10")
        },
        _ => {
            println!("match -infinite -> 0 or 11 -> +infinite")
        }
    }
}

// Fill the blank to make the code work
pub fn run2() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        (first, .., last) => { // bỏ qua các giá trị ở giữa
           assert_eq!(first, 2);
           assert_eq!(last, 2048);
        }
    }

    println!("Success!");
}



pub fn run3(){
    let num = Some(4);
    let split = 5;
    match num {
        // Fill in the blank to make the code work, `split` MUST be used
        Some(x) if x < split => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }

    println!("Success!");
}

pub fn run4() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        (first, .., last) => { // bỏ qua các giá trị ở giữa, chỉ lấy giá trị đầu tiên và cuối cùng
           assert_eq!(first, 2);
           assert_eq!(last, 2048);
        }
    }

    println!("Success!");
}

pub fn run5(){
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
       value => value.push_str(" world!") 
    }
}