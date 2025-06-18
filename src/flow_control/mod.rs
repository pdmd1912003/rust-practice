
// Fix the errors
pub fn run1() {
    let n = 5;

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            10 * n
        } else {
            println!(", and is a big number, halve the number");

            n / 2
        };

    println!("{} -> {}", n, big_n);
} 

pub fn run2(){
    for n in 1..100 { // modify this line to make the code work // 
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }

    println!("Success!");
}
pub fn run3() {
    let names = [String::from("liming"),String::from("hanmeimei")];
    for name in &names {
        // Do something with name...
    }

    println!("{:?}", names);

    let numbers = [1, 2, 3];
    // The elements in numbers are Copy，so there is no move here
    for n in numbers {
        // Do something with n...
    }
    
    println!("{:?}", numbers);
}

pub fn run4() {
    let a = [4, 3, 2, 1];

    // Iterate the indexing and value in 'a'
    for (i,v) in a.iter().enumerate() { // iter() trả về iterator, enumerate() gói nó và trả về một tuple (index, value) 
        println!("The {}th element is {}",i+1,v);
    }
}

pub fn run5() {
   let mut n = 0;
    for i in 0..=100 {
       if n != 66 {
           n+=1;
           continue; // continue bỏ qua phần còn lại của vòng lặp hiện tại và bắt đầu vòng lặp tiếp theo đến khi n == 66
       }
       
       break; // break kết thúc vòng lặp ngay lập tức
    }

    assert_eq!(n, 66);

    println!("Success!");
}   

pub fn run6() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // lặp đến khi counter == 10, sau đó trả về giá trị counter * 2
        }
    };

    assert_eq!(result, 20); // kiểm tra xem kết quả có bằng 20 không

    println!("Success!");
}