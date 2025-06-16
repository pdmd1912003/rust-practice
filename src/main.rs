mod variables;
mod basic_types{
    pub mod numbers;
}
fn main() {
    println!("Hello, world!");
    basic_types::numbers::run1();
    basic_types::numbers::run2();
    basic_types::numbers::run3();
    basic_types::numbers::run4();
    basic_types::numbers::run6();
    basic_types::numbers::run8();
}
