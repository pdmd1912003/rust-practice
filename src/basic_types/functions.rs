use core::panic;

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

pub fn run1() {
   let (x, y) = (1, 2);
    let s = sum(x, y);
    assert_eq!(s, 3);

    println!("Success!");
}

pub fn run2() {
    print();
}
fn print(){
    println!("Success!");
}


// Solve it in two ways
// DON'T let `println!` work
pub fn run3() {
    never_return();
    println!("Failed!");
}

//way 1
fn never_return() -> ! {
    loop{}
    
}
//way 2
fn never_return_2() -> ! {
    panic!("This function never returns!");
}

//Diverging functions
pub fn run4() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };
    never_return_fn()
}

fn never_return_fn() -> ! {
    panic!("Never return function")
}