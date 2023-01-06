/* iterator type and
 map(), filter(), fold()
 iterators let you perform some task on each item in a sequence of items in turn
 iterators implement Iterator trait
 iterators, iterator adaptors (map, filter), iterator consumers (collect, fold, sum, any, find)
 iterator adaptors produce other iterators
 iterator consumers call next()
*/

use factorial::Factorial;                                   // dependency in cargo.toml on factoral package
// factorial crate

fn main() {

    // ranges
    for i in 1..10 {                                                    // 1 to 9
        print! {"{}! = {}, ", i, (i as u32).factorial()};               // 1! to 9!
    }
    println!{};

    // vectors are allocated on heap, push, pop, [], implement Iterator trait
    // vectors are re-sizeable arrays
    let data: Vec<i32> = vec![1, 2, 3, 4];                  // or (1..5).collect(), ! macro
                                                            // (1..5) is a range which is an iterator
    for i in data.iter() {                                  // get iterator for data, avoid moving, iterators are lazily evaluated
        print! {"{} ", i};
    }

    // .iter() produces an iteration over immutable references (also into_iter() and iter_mut())
    assert_eq!(data.iter().next(), Some(&1));               // or None, Option<i32>

    let total: i32 = data.iter().sum();                     // calls next, sum is a consumer
    assert_eq!(total, 1 + 2 + 3 + 4);

    // map x * x to each element x and collect results
    let squares = (1..=5).map(| x | x * x).collect::<Vec<u32>>();
    println! {"squares {:?}", squares};
    assert_eq!(squares, vec![1, 4, 9, 16, 25]);

    // filter odd numbers into a new vector, filter is a iterator adaptor
    let odds = (1..=5).filter(| x | x % 2 != 0).collect::<Vec<u32>>();
    println! {"odds {:?}", odds};

    // fold into a product of all items in data, fold is a consumer
    let product = (1..=5).fold(1, | total, x | total * x);
    println!("product {}", product);

    // 3 to 8
    let data: Vec<u32> = (3..9).collect();

    // factorial crate dependency, map gives closure ownership of iterator elements
    print!{"! {:?} \n", data.iter().map(| x | x.factorial()).collect::<Vec<u32>>()};
}


// common macros:
// vec!                 - make a vector with values/repeating values
// panic!               - make current thread panic
// assert!              - takes a bool
// assert_eq!           - takes 2 params
