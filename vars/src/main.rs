// variable bindings and types (scalar and compound)
// Rust is statically typed

#![allow(unused_variables)]

pub const MAX: u32 = 99;                    // cannot be initialised to an expression, cannot be shadowed

fn main() {
    let a = 8;                              // immutable, variable binding called a to value 8, type inference
                                            // let is a declaration statement
    println! {"{}", i32::MAX};              // i32 module in std crate

    // b is a binding to a u64 type and the value 1
    let mut b: u64 = 1;                     // mutable, type annotation, no type inference
    b = 2;
    let b = "b";                            // shadow a variable binding
    println!("{}", b);

    let c: &str;
    c = "10";                               // &str string slice
    dbg!(c);

    let mut d = {
        let e = 1;
        e + 2                               // expression, no ;
    };
    println!("{}", d);

    // Rust is expression based, expressions return a value, statements do not
    d = if a < 9 { 1 } else { 10 };

    // call fns
    d = increment1(d);
    dbg!(d);
    increment2(&mut d);
    dbg!(d);

    // tuples
    let (x, y) = (1, 2);
    dbg!(x);
    dbg!(y);

    let c: char = 'c';                      // UTF-32 i.e. 4 bytes
    let message: &str = "finished";
    let finished: bool = true;

    // literals
    // 4                        i32
    // 4.0                      f64
    // isize and usize depend on system architecture i.e 32 bits or 64 bits

    // signed v unsigned
    // 2 bits signed -8 .. 7
    // 4 bits unsigned 0 .. 15
}

// ++
fn increment1(x: i32) -> i32 {
    // in a function use "return" for an early return
    x + 1                                   // or return x + 1;             // poor style
}

// ++
fn increment2(x: &mut i32) {
    *x += 1;
}

// primitive types:
// fixed size types for integers:
// i or u, 8, 16, 32, 64, 128
// i32 is default
// overflow checking is enforced at runtime if compiled in debug mode, program will panic

// variable size types for integers:
// isize and usize (architecture dependent)

// floating point numbers, IEEE 754
// f32 (single precision) or f64 (double precision)
// f64 is default

// char (4 bytes, Unicode)
// bool

// sequences types:
// arrays [1, 2, 3]                         same types in a collection, fixed size
// tuples (1, true)                         different types in a collection, fixed size
// slices

// user defined types:
// structs
// enums

// also function types, trait types, pointer types

// cargo build
// cargo build --release
// cargo clippy
// cargo fmt
// cargo check
// cargo run
