
// Fix the errors
enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One,
    Two,
}

// C-like enum
// enum Number2 {
//     Zero = 0.0,
//     One = 1.0,
//     Two = 2.0,
// }
//-> không gán giá trị enum là số thực, chỉ có thể là số nguyên

pub fn run1() {
    // An enum variant can be converted to a integer by `as`
    assert_eq!(Number::One as u8, Number1::One as u8); // -> ép kiểu enum sang cùng kiểu để so sánh

    println!("Success!");
} 


pub fn run2() {
    let msg1 = Message::Move{x : 1, y : 2}; // Instantiating with x = 1, y = 2 
    let msg2 = Message::Write("hello, world".to_string()); // Instantiating with "hello, world!"

    println!("Success!");
} 


// Fill in the blank and fix the error
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub fn run3() {
    let msg = Message::Move{x: 1, y: 2};

    if let Message::Move{x:a, y:b} = msg {
        assert_eq!(a, 1);
        assert_eq!(b, 2);
    } else {
        panic!("NEVER LET THIS RUN！");
    }

    println!("Success!");
} 

pub fn run4() {
    let msgs:[Message; 3]  = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }
} 

fn show_message(msg: Message) {
    println!("{:?}", msg);
}

pub fn run5() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let Some(n) = six { 
        println!("{}", n);
        return; // -> nếu có giá trị thì in ra và kết thúc hàm
    } 
        
    panic!("NEVER LET THIS RUN！");
} 

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}