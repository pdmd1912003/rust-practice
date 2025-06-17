pub fn run1(){
    // Use as many approaches as you can to make it work
    let x = String::from("Hello world");
    let y = x.clone();
    println!("{}, {}",x, y);
}

// Don't modify code in main!
pub fn run2() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

fn take_ownership(s: String) -> String {
    s
}


pub fn run3() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("Hello world");
    // Convert String to Vec
    let _s = s.clone().into_bytes(); // -> clone xong roi moi move 
    s
}

// Fix the error without removing any code
pub fn run4() {
    let s = String::from("Hello World");
    print_str(s.clone()); // clone de tranh bi mat ownership

    println!("{}", s);
}

fn print_str(s: String)  {
    println!("{}",s)
}

// Don't use clone ,use copy instead
pub fn run5() {
    let x = (1, 2, (), "hello"); // -> thay doi String bang copy type
    let y = x; 
    println!("{:?}, {:?}", x, y);
}

//Mutability

// make the necessary variable mutable
pub fn run6() {
    let s = String::from("Hello ");
    
    let mut s1 = s; // -> s1 is mutable

    s1.push_str("World!");

    println!("Success!");
}

pub fn run7() {
    let x = Box::new(5);

    let mut y = x.clone(); // update this line, don't change other lines!

    *y = 4;
    
    assert_eq!(*x, 5);

    println!("Success!");
}

pub fn run8(){
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }
    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    let Person { name, ref age } = person; // -> sử dụng ref để mượn age thay vì move
    println!("Person age is : {}", age);
    println!("Person name is : {}", name);
    // println!("Person is : {:?}", person); -> lỗi bởi vì person đã move 1 phần
    // person.age vẫn sử dụng được vì nó được mượn bằng ref chứ không move như name
    println!("Person age from struct is : {}", person.age);
}
pub fn run9(){
    let t = (String::from("hello"), String::from("world"));

   let _s = t.0;

   // Modify this line only, don't use `_s`
   println!("{:?}", t.1); // -> t.1 vẫn sử dụng được vì chỉ t.0 bị move
}

pub fn run10() {
    let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (s1, s2) = t.clone(); // -> clone để tránh move, s1 và s2 sẽ là các bản sao của t.0 và t.1

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}
