
// binding and mutability

//🌟 A variable can be used only if it has been initialized.
pub fn run(){
    let x = 5;
    let y = 10;
    
    assert_eq!(x, 5);
    println!("Success")
}
//🌟 Use mut to mark a variable as mutable.
pub fn run2(){
    let mut x = 1;
    x += 2;
    assert_eq!(x, 3);
    println!("Success");
}
//A scope is the range within the program for which the item is valid.
//🌟
pub fn run3(){
    let x = 10;
    let y = 5;
    {
        println!("Inner scope: x = {}, y = {}", x, y);
    }
    println!("Outer scope: x = {}, y = {}", x, y);
}

//🌟🌟
// Fix the error with the use of define_x
pub fn run4() {
    println!("{}, world", define_x()); 
}

fn define_x() -> &'static str {
    let x = "Hello";
    x
}


//Shadowing

//🌟🌟
pub fn run5(){
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
}
//🌟🌟
pub fn run6(){
     let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    x += 3;

    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!"; 

    println!("Success!");
}

pub fn run7(){
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

//Destructuring
pub fn run8 () {
     let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    assert_eq!([x,y], [3, 2]);

    println!("Success!");
}

