// ownership and borrowing

fn main() {
   ownership();
   borrow();
   mut_borrow()
}

fn ownership() {
    let s1 = String::from("hello");     // on heap, s1 owns the string
    let s2 = s1;                        // a move, now s2 owns the string
    println!("{}", s2);                 // println!("{}", s1} won't compile, clone an alternative
}

fn borrow() {
    let s1 = String::from("hello");     // on heap, s1 owns the string
    let s2 = &s1;                       // s2 is a immutable borrow of s1, can have several
    let s3 = &s2;
    println!("{} {} {}", s1, s2, s3);            
}

fn mut_borrow() {
    let mut s1 = String::from("hello");     // on heap, s1 owns the string, mutable
    let s2 = &mut s1;                       // s2 is a mutable borrow of s1, there can be only one at a time
    s2.push('!');
    println!("{}", s2);                     // s2 released after this line of code               
    println!("{}", s1);        
}

