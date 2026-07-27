//A function pointer (fn) is a type that stores the address of a function. It allows you to pass a regular function as an argument to another function.
//fn is a type, whereas Fn, FnMut, and FnOnce are traits for closures.

//passing functions as arguments  => Rust allows you to pass named functions as arguments, just like closures.

//function poiter vs closureee
//A function pointer refers to an existing function, while a closure is an anonymous function that can capture variables from its environment.

//using functions with map => Instead of writing a closure, you can pass a function pointer directly to methods like map().
/*
fn main() {
    let numbers = vec![1, 2, 3];
    let strings: Vec<String> =
        numbers.iter().map(|x| x.to_string()).collect();
    println!("{:?}", strings);
}*/

//Enum variant as function pointerr
//Each enum variant behaves like a constructor function, so it can be passed where a function or closure is expected.
//Closures have anonymous types, so they cannot be returned directly. Instead, return them using impl Fn.
/*
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}
fn main() {
    let add = returns_closure();

    println!("{}", add(5));
}*/

//Use the move keyword when the closure captures variables from its surrounding environment.
/*
fn make_adder(value: i32) -> impl Fn(i32) -> i32 {
    move |x| x + value
}
fn main() {
    let add10 = make_adder(10);

    println!("{}", add10(5));
}*/

//Returning multiple closuressssssssssss
//Each closure has its own unique type. If different functions return different closures, use Box<dyn Fn> so they share the same return type.

//fn is a function pointer type for named functions.
//Fn is a trait implemented by closures (and also by function pointers).
//se Box<dyn Fn> when you need to return or store different closure types behind a common interface, such as in a Vec.