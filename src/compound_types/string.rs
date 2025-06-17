pub fn run1(){
    let s: &str = "hello, world"; // -> 

    println!("Success!");
}

// Fix the error with at least two solutions
pub fn run2() {
    let s: Box<str> = "hello, world".into();
    greetings(s);
    //greetings2(&s);
}
//solution 1: use Box<str> to convert &str to Box<str>
fn greetings(s: Box<str>) {
    println!("{}",s)
}

//solution 2: use String to convert &str to String
fn greetings2(s: &str) {
    println!("{}",s)
}

pub fn run3() {
    let mut s = String::new(); // -> use String instead of &str
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}
// Fix all errors without adding newline
pub fn run4() {
   let mut  s = String::from("hello");
    s.push(',');
    s.push_str("world"); // -> dùng push_str để add string
    s += "!";

    println!("{}", s);
}

pub fn run5() {
    let s = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let s1 = s.replace("dogs", "cats"); // -> dùng replace để thay thế "dogs" bằng "cats"

    assert_eq!(s1, "I like cats");

    println!("Success!");
}

pub fn run6() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // -> nối hai chuỗi, s1 sẽ bị move sang s3, s2 sẽ không bị move vì nó là tham chiếu
    assert_eq!(s3, "hello,world!");
    println!("{}", s3);
}

//&str and String
pub fn run7() {
    let s = "hello, world";
    greetings3(String::from(s)); // solution 1: đổi &str sang String bằng String::from
    //greetings3(s.to_string()); // solution 2: đổi &str sang String bằng to_string()
}

fn greetings3(s: String) {
    println!("{}", s)
}

//String escapes
//
pub fn run8() {
let raw_str = "Escapes don't work here: \x3F \u{211D}";
    // Modify above line to make it work
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // If you need quotes in a raw string, add a pair of #s
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // If you need "# in your string, just use more #s in the delimiter.
    // You can use up to 65535 #s.
    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // Fill the blank
    let long_delimiter = r###"Hello, "##""###; // vì "##" không phải là ký tự đặc biệt trong chuỗi thô, nên nó sẽ được giữ nguyên
    // long_delimiter sẽ là "Hello, "##""
    assert_eq!(long_delimiter, "Hello, \"##\""); 

    println!("Success!");
}
pub fn run9() {
    // Fill the blank to print each char in "你好，世界"
    for c in "你好，世界".chars(){ // -> sử dụng chars() để lấy từng ký tự trong chuỗi
        println!("{}", c)
    }
}
use utf8_slice;
pub fn run10(){
    let s = "The 🚀 goes to the 🌑!";

    let rocket = utf8_slice::slice(s, 4, 5);
    // Will equal "🚀"
    println!("Rocket: {}", rocket);
}