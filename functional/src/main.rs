// functional programming
// functions are 1st class citizens in Rust
// a variable binding can bind to a function i.e. a function pointer

fn main() {
    let hello = hello_world1;                       // store a functional in a variable, hello could be mut
    println! {"{}", hello()}

    // re-bind, this time without type inference
    let hello: fn() -> String = hello_world1;

    // array of functions
    let hello_all = [hello_world1, hello_world2];
    for h in hello_all.iter() {
        println! {"{}", h()};
    }

    // call pure fn
    println! {"{}", add(10, 20)};               // 30

    let mut y = 20;
    increment(&mut y);                          // not pure
    println! {"{}", y};                         // 21

    // anonymous fns and closures (aka lambdas in C#, Python)

    // u32
    // f64

    let add = |a: u32, b: u32| a + b;       // anonymous fn, compiler deduces return type
    println! {"{}", add(10, 20)};

    let increment = |i: u32| i + 1;         // can use {} for body if multiple statements
    println! {"{}", increment(10)};

    let hello = || "hello world";
    println! {"{}", hello()};

    // closure, closure will capture variables

    let message = String::from("hello world");

    // this is a closure since its "closes" around message in surronding scope
    let hello = || message + " closure";
    println! {"{}", hello()};
    
    // message is now owned by the closure (moved)
    
    // anonymous fns are closures with empty scope

}

// pure function - no side effect
fn hello_world1() -> String {
    String::from("hello world 1")
}


// pure function - no side effect
fn hello_world2() -> String {
    String::from("hello world 2")
}

// pure function, output is fn of inputs
fn add(x: u32, y: u32) -> u32 {
    x + y
}


// not pure, has side effect
fn increment(x: &mut u32) {
    // mutable reference
    *x += 1;
}


// a function can also return a function

// a pure function that takes no parameters must return a constant - so is pretty useless
// a pure function takes returns nothing is pretty useless
// useful pure functions have one or more parameters and return a value
