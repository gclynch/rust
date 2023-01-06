// structs

fn main() {
    let r1 = Rectangle {
        width: 10,
        height: 10,
    };
    let r2 = Rectangle { 
        height: 20,
        ..r1 };            // same width as r1
        
    println!("{:?}", r2);
    println!("{}", r2.area());
    println!("{}", r2.can_hold(&r1));           // borrow r1

    // make a square
    let s: Rectangle = Rectangle::square(10);
    println!("{:?}", s);
    assert_eq!(s.area(), 10 * 10);             
    
    // ! macro, macros are very flexible, and expand to code based on an invocation
    // use as a last resort for code re-use
}

// attribute
#[derive(Debug)]                                // compiler provides basic implementation of trait Debug i.e. {:?} pretty printing
struct Rectangle {
    width: u32,                                 // field
    height: u32,
}

// implementation block
impl Rectangle {                                                            // can have several impl blocks for same struct
    // can self hold other
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // assoicated fn, no self therefore
    fn square(x: u32) -> Rectangle {
        Rectangle {
            width: x,
            height: x,
        }
    }

    //&self - taking self by reference (borrow)
    //&mut self - taking self by mutable reference (borrow)
    //self - taking ownership of self

    // associated fns (static in C#) have no self parameter
}

// a trait
trait HasArea {
    fn area(&self) -> u32;                      // just define method signature
}

impl HasArea for Rectangle {
    // calc area of rectangle
    fn area(&self) -> u32                       // borrowed
    {
        self.width * self.height
    }
}

