use std::mem::size_of_val;


pub fn run1(){
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4); 

    let c2 = '中';
    assert_eq!(size_of_val(&c2),4); 

    println!("Success!");
}

pub fn run2() {
    let c = '中';
    print_char(c);
}

fn print_char(c: char) {
    println!("{}", c);
}

pub fn run3() {
   let _f: bool = false;

    let t = true;
    if t {
        println!("Success!");
    }
}

pub fn run4() {
    let f = true;
    let t = true || false;
    assert_eq!(t, f);

    println!("Success!");
}

// Make it work, don't modify `implicitly_ret_unit` !
pub fn run5() {
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}

pub fn run6() {
     let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!");
}