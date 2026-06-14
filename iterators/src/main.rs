//in rust iterators are lazy, meaning they do not do anything until you call a method that consumes the iterator. This allows for efficient processing of large data sets, as you can chain together multiple iterator methods without creating intermediate collections.
/*
fn main() {    
  let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();// creates an iterator over the vector v1

    for val in v1_iter {
        println!("Got: {val}");
    }
}*/

//The next() method returns one item at a time wrapped inside Some(). When the sequence ends, it returns None.
/*
fn main() {
    let v1 = vec![1, 2, 3];
    let mut iter = v1.iter();
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
}*/

//Consuming adapters consume the iterator and produce a final value.
/*
fn main() {
    let v = vec![1, 2, 3];
    let iter = v.iter();
    let total: i32 = iter.sum();// the sum() method consumes the iterator and returns the sum of the elements
    println!("{}", total);
}*/

//Iterator adapters transform one iterator into another without consuming it.
/*
fn main() {
    let v = vec![1, 2, 3];
    let new_iter = v.iter().map(|x| x + 1);
}*/


//collect() consumes an iterator and gathers the results into a collection.

/*  
fn main() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> =
        v1.iter()
          .map(|x| x + 1)
          .collect();
    println!("{:?}", v2);
}  */


//filter() keeps only elements for which the closure returns true.
/*
fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let even: Vec<_> =
        nums.into_iter()
            .filter(|x| x % 2 == 0)
            .collect();
    println!("{:?}", even);
}   */

//Closures can capture values from their environment. In this example, the closure captures the shoe_size variable from the surrounding scope.
/*
fn main() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
    ];
    let shoe_size = 10;
    let shoes = shoes_in_size(shoes, shoe_size);
    println!("{:?}", shoes);
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Shoe {
    size: u32,
    style: String,
}
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()

         // Closure captures shoe_size
         .filter(|shoe| shoe.size == shoe_size)

         .collect()
}*/

// iter()      → Borrows elements immutably (&T), collection remains usable.
// iter_mut()  → Borrows elements mutably (&mut T), allowing modification.
// into_iter() → Takes ownership of elements (T), consuming the collection.
fn main() {
    let mut numbers = vec![1, 2, 3];
        // iter() → immutable references (&i32)
    for x in numbers.iter() {
        println!("iter(): {}", x);
    }
    // iter_mut() → mutable references (&mut i32)
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("After iter_mut(): {:?}", numbers);

    // into_iter() → takes ownership (i32)
    for x in numbers.into_iter() {
        println!("into_iter(): {}", x);
    }
    // println!("{:?}", numbers); // ERROR: numbers has been moved
}