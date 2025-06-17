pub fn run1(){
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

// Make it work with two ways

// way 1:
pub fn run2() {
    let v ={
        let mut x = 1;
        x += 2;
        x
    };
    println!("Success!");
}

// way 2:
pub fn run3() {
    let mut x = 1;
    x += 2;
    let v = return_value(x);
    assert_eq!(v, 3);
    println!("Success!");
}
fn return_value(x: i32) -> i32 {
    x
}

