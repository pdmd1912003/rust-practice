//We must specify concrete values for each of the fields in struct.

struct Person {
    name: String,
    age: u16,
    hobby: String,
}

pub fn run1(){
    let age = 30;
    let person = Person {
        name: String::from("Alice"),
        age, // using the variable `age` directly
        hobby: String::from("Reading"),
    };
    println!("Person name: {}, age: {}, hobby: {}", person.name, person.age, person.hobby);
}

struct Unit;
trait SomeTrait {
    // ...Some behaviors defined here.
}

// We don't care about what fields  are  in the Unit, but we care about its behaviors.
// So we use a struct with no fields and implement some behaviors for it
impl SomeTrait for Unit {  }
pub fn run2() {
    let u = Unit;
    do_something_with_unit(u);

    println!("Success!");
} 

// Fill the blank to make the code work
fn do_something_with_unit(u: Unit) {   }


struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
pub fn run3() {
    let v = Color(0, 127, 255); // -> dùng đúng Color struct
    check_color(v);

    println!("Success!");
}   

fn check_color(p: Color) {
    let Color(x, _, _) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(p.2, 255);
 }

 struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn run4() {
    let  u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(u1);

    println!("Success!");
} 

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        username: u.username,
        active: u.active,
        sign_in_count: u.sign_in_count,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub  fn run6() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // Print debug info to stderr and assign the value of  `30 * scale` to `width`
        height: 50,
    };

    dbg!(&rect1); // Print debug info to stderr

    println!("rect1: {:?}", rect1); // Print debug info to stdout
}


pub fn run7() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved out of person, but `age` is referenced
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person` partial move occurs
    //println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("The person's age from person struct is {}", person.age);
}