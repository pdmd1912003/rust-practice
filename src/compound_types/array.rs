pub fn run1(){
     // Fill the blank with proper array type
    let arr: [i32; 5]= [1, 2, 3, 4, 5]; // [i32; 5]

    // Modify the code below to make it work
    assert!(arr.len() == 5);

    println!("Success!");
}

// fill the blank 
pub fn run2(){
    let list: [i32; 100] = [1;100] ;  

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("Success!");
}

pub fn run3(){
    let names = [String::from("Sunfei"), "Sunface".to_string()];
    
    // `Get` returns an Option<&T>, it's safe to use
    let name0 = names.get(0).unwrap();

    // But indexing is not safe
    let _name1 = &names[1]; // This is safe because we know the index is valid

    println!("Success!");
}