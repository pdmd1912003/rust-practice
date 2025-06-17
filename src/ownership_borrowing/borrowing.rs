// reference

pub fn run1(){
    let x = 5;
   // Fill the blank
   let p = &x;

   println!("the memory address of x is {:p}", p); // -> the memory address of x is 0x7ffec85a8f7c
}

pub fn run2() {
    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(5, *y); // dereference y để lấy giá trị 

    println!("Success!");
}

pub fn run3() {
    let mut s = String::from("hello, ");

    borrow_object(&s); // borrow_object nhận tham chiếu đến s, không phải ownership

    println!("Success!");
}

fn borrow_object(s: &String) {}

pub fn run4() {
    let mut s = String::from("hello, ");

    push_str(&mut s); // push_str nhận tham chiếu mut đến s, cho phép thay đổi nội dung của s

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}


pub  fn run5() {
    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    let p = &mut s; // p là một tham chiếu mutable đến s
    
    p.push_str("world");

    println!("Success!");
}

// Ref

pub fn run6() {
    let c = '中';

    let r1 = &c;
    // Fill the blank，dont change other code
    let ref r2 = c; // r2 là một tham chiếu đến c 

    assert_eq!(*r1, *r2); // dereference r1 và r2 để so sánh giá trị của chúng
    
    assert_eq!(get_addr(r1),get_addr(r2)); // địa chỉ của r1 và r2 giống nhau vì chúng đều tham chiếu đến cùng một giá trị c

    println!("Success!");
}

// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

// Borrowing rules
// Remove something to make it work
// Don't remove a whole line !
pub  fn run7() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s; // vi phạm quy tắc mượn, chỉ cho phép một tham chiếu mutable tại một thời điểm
    println!("{}", r1);

    println!("Success!");
}

// Error: Borrow an immutable object as mutable
pub fn run9() {
    // Fix error by modifying this line
    let mut s = String::from("hello, "); // thêm mut để cho phép thay đổi nội dung của s

    borrow_object(&mut s);

    println!("Success!");
}

//NLL

// Comment one line to make it work
pub fn run10() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    /*let r2 = &mut s; // vì r1 đang mượn s, nên không thể mượn s một lần nữa bằng r2
    r2.push_str("!");*/
    
    println!("{}",r1);
}

pub fn run11() {
     let mut s = String::from("hello, ");

    let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2); // -> Error: dùng đồng thời hai &mut
}



