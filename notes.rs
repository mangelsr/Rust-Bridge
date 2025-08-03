// main.rs -> Comments with double slash
// fn -> Keyword to declare function
// {} -> Use curly braces to define blocks
// main -> Function name -> define the start of the program
// println!
// ""  -> &str datatype
// ; -> end of line statement
// ! -> define is a macro
fn main() {
    println!("Hello World!");

    let x; // Declare "x"
    x = 42; // assign 42 to x

    let x = 42;
    // declare and assign on the same line
    // Rust is an strict typed language, but it can inffer the data types of the variables

    // Type annotations
    let x: i32 = 42;

    // i32 is an 32 bits integer (with sign)

    // i8, i16, i32, i 64, i128
    // y u8, u16, u32, u64, u128 (sin signo)

    // Tuples, collections of fixed length of different data types
    let par = ('a', 17);
    par.0; // This is 'a'
    par.1; // This is 17

    // Tuples destructuration
    let (a_char, a_int) = ('a', 17);

    // let (left, rigth) = slice.split_at(middle_idx);

    // Statements
    let x = vec![1, 2, 3, 4, 5, 6, 7, 8]
        .iter()
        .map(|x| x + 3)
        .fold(0, |x, y| x + y);

    fn toss_dice() -> i32 {
        4 // This is an expresion -> return values
    }

    // Blocks are expresions
    let x = 42;
    let x = { 42 };

    // Both lines are equivalent

    let x = {
        let y = 1; // first statment
        let z = 2; // last statement
        y + z // Last expresion (not a statement) THIS IS AN IMPLICIT RETURN
    };

    // When making early returns; its mandatory to use EXPLICIT RETURN

    // IN RUST ALL IS A EXPRESION
    // Rust dont have ternary operator
    fn toss_dice() -> i32 {
        if fell_luck {
            6
        } else {
            4
        }
    }

    // Pattern Matching
    // Match only compile if all the cases are covered
    // Match is EXAUSTIVE
    fn toss_dice() -> i32 {
        match fell_luck {
            true => 6,
            false => 4,
        }
    }

    // Keyword use
    let min = std::cmp::min(3, 8);

    // Here we are importing
    // use std::cmp::min;
    let min = min(7, 1);

    // :: vs .
    // The types are also namespaces and the methods cal be called like regular functions

    let x = "miguel".len(); // this is 6
    let x = str::len("miguel"); // this is also 6

    // Prelude
    // use std::prelude::v1::*;

    let v = Vec::new();

    // Structs
    struct Vec2 {
        // Floating point of 64 bits ("Double presision")
        x: f64;
        y: f64;
    }

    let v1 = Vec2{ x: 1.0, y: 3.0 };
    let v1 = Vec2{ y: 2.0, x: 4.0 };
    // The order is not mandatory

    // Destructure structs
    let Vec2 { x, y } = v1;
    let Vec2 { x, .. } = v2;
    
    // The _ is only for one thing
    // The .. operator discard the rest


    // Jump Table -> Research

    // Patron Catch All
    // _ =>

    // Deteccion de rangos en match usar 
    // start..end (end not included)
    // start..=end (end included)


    // Methods
    struct Number {
        even: bool;
        value: i32;
    }

    // impl keyword
    impl Number {
        fn is_positive(self) -> bool {
            self.value > 0
        }
    }

    // Rust is not considered an OO PL, it didn't have Inheritance
    // It have Encapsulation, Abstraction, and Polimorfism

    // Inmutability
    // Every in RUST is INMUTABLE by default
    // Keyword mut

    let mut n = Number { even: true, value: 17 };
    n.value = 19;


    // Ownership
    // Central memory management concept of Rust
    // All value have only one owner
    // Propietary exists the "scope" -> the value gets dropped (the memory is free) 
    // Avoid "use after free"

    // Borrowing
    // Allow tempral access to data without let ownership
    // Use references (& to share, &mut to mutate)
    // Verified by the compiler (the "borrow checker") 
    // "data races"
    // Garantee the security of the Data
    // - Is muttalbe or is share (XOR)
    // - The references "cant live long" that their respectives data 

    // Ownership and Borrowing implecations
    // Memory Security without Garbage Collector
    // Concurency security
    // Performances


}
