mod variables;
mod basic_types{
    pub mod numbers;
    pub mod char_bool_unit;
    pub mod statements_expression;
    pub mod functions;
}
mod ownership_borrowing {
    pub mod ownership;
    pub mod borrowing;
}
mod compound_types {
    pub mod tuple;
    pub mod array;
    pub mod slice;
    pub mod string;
    pub mod structs;
    pub mod enums;
    // pub mod generics;
}
fn main() {
    // basic_types::numbers::run1();
    // basic_types::numbers::run2();
    // basic_types::numbers::run3();
    // basic_types::numbers::run4();
    // basic_types::numbers::run6();
    // basic_types::numbers::run8();

    // basic_types::char_bool_unit::run1();
    // basic_types::char_bool_unit::run2();

    // basic_types::statements_expression::run1();
    // basic_types::statements_expression::run3();
    // basic_types::functions::run1();
    // basic_types::functions::run2();
    // ownership_borrowing::ownership::run1();
    // ownership_borrowing::ownership::run5();
    // ownership_borrowing::ownership::run8();
    // compound_types::string::run10();
    // compound_types::array::run2();
    // compound_types::array::run3();
    // compound_types::slice::run1();
    // compound_types::slice::run3();
    // compound_types::tuple::run2();
    // compound_types::structs::run3();
    compound_types::enums::run1();
    compound_types::enums::run2();
    compound_types::enums::run3();
    compound_types::enums::run4();

}
