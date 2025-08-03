# Rust Comments and Concepts

## Comments

main.rs -> Comments with double slash  
Single-line comments are written using `//`

---

## Functions and Syntax Basics

`fn` -> Keyword to declare a function  
Used to define functions

`main` -> Function name; defines the program's entry point

`{}` -> Curly braces define code blocks (functions, conditionals, loops, etc.)

`println!`
A macro that prints text to the console

`""` -> &str data type
Represents a string literal, a reference to a string

`;` -> End of statement
Used to terminate most statements

`!` -> Indicates it's a macro, not a regular function

---

## Variables and Types

`let x;` Declare "x" without initializing

`x = 42;` Assign 42 to x afterward

`let x = 42;`
Declare and initialize on the same line

Rust is a statically typed language
But it can infer the type of variables automatically

Type annotations
Explicitly specify the type of a variable

`let x: i32 = 42;`
Specifies `x` is a signed 32-bit integer

i8, i16, i32, i64, i128
Signed integer types

u8, u16, u32, u64, u128
Unsigned integer types

---

## Tuples

Tuples: collections of fixed length with different data types
`let pair = ('a', 17);`

`pair.0;` Access the first element ('a')
`pair.1;` Access the second element (17)

Destructuring tuples
`let (a_char, a_int) = ('a', 17);`
Assigns `'a'` to `a_char` and `17` to `a_int`

---

## Expressions and Statements

Rust treats almost everything as an expression

`let x = vec![1,2,3].iter().map(|x| x+3).fold(0, |x,y| x+y);`
Creates a vector, iterates, maps, and folds (reduces) it

```rust
fn toss_dice() -> i32 {
    4
}
```

The return value is the last expression (no semicolon needed)

---

## Blocks as Expressions

`let x = { 42 };`
A block can return a value

```rust
let x = {
    let y = 1;
    let z = 2;
    y + z // Last expression: implicitly returned
};
```

To return early within a block or function,
use the `return` keyword explicitly

---

## Conditionals

Rust does not have a ternary operator
if/else itself is an expression

```rust
fn toss_dice() -> i32 {
    if feel_lucky { 6 }
    else { 4 }
}
```

---

## Pattern Matching

match is exhaustive: it must cover all possible cases

```rust
fn toss_dice() -> i32 {
    match feel_lucky {
        true => 6,
        false => 4,
    }
}
```

`_` => Catch-all pattern for anything not explicitly matched

Ranges in match:
`start..end` up to but excluding `end`
`start..=end` up to and including `end`

---

## Modules and Imports

`let min = std::cmp::min(3, 8);`
Full namespace path

`use std::cmp::min;`
Imports the function so you can call `min(7, 1)` directly

`::` vs `.`
`::` is used for namespaces, modules, and types
`.` is used to call instance methods

`"miguel".len()` -> method called on an instance
`str::len("miguel")` -> associated function called on type

---

## Prelude

`std::prelude::v1::\*`
Imported by default, giving access to common types like Vec, String, etc.

---

## Structs

`struct Vec2 { x: f64; y: f64; }`
Defines a structure

`let v1 = Vec2 { x: 1.0, y: 3.0 };`
The order of fields in initialization doesn't matter

Destructuring structs
`let Vec2 { x, y } = v1;` get both fields
`let Vec2 { x, .. } = v2;` get only `x`, ignore the rest with `..`

---

## Methods and `impl`

```rust
struct Number { even: bool; value: i32; }

impl Number {
    fn is_positive(self) -> bool {
    self.value > 0
    }
}
```

`impl` is used to associate methods with a struct

Rust is not an object-oriented language in the classic sense
It doesn't have inheritance
But it supports encapsulation, abstraction, and polymorphism (via traits)

---

## Mutability

By default, everything in Rust is immutable

`let mut n = Number { even: true, value: 17 };`
`n.value = 19;` Only possible because of `mut`

---

## Ownership

Each value has exactly one owner
When the owner goes out of scope, the value is dropped and memory is freed
Prevents errors like "use after free"

---

## Borrowing

Allows temporary access to data without transferring ownership
& creates a shared reference, &mut creates a mutable reference

Checked by the compiler's "borrow checker"
Prevents data races
Guarantees data safety

A reference cannot outlive the data it points to

---

## Summary

Ownership + Borrowing
-> Memory safety without a garbage collector
-> Safe concurrency
-> High performance
