//The Newtype Pattern is a technique where you create a new type by wrapping an existing type inside a tuple struct.
/*Why use it?
Type safety
Hide implementation details
Implement external traits (Orphan Rule)
Represent different units*/
/*
struct Kilometers(i32);
fn print_distance(d: Kilometers) {
    println!("Distance = {} km", d.0);
}
fn main() {
    let d = Kilometers(50);
    print_distance(d);
    // print_distance(50); Compile Error
}*/

//Type Alias (Type synonym)

//A Type Alias gives another name to an existing type.It does not create a new type.
/*type Kilometers = i32;
fn main() {
    let x: i32 = 10;
    let y: Kilometers = 20;
    println!("{}", x + y);
}*/
/*Newtype	                    Type Alias
Creates a new type	            Creates another name
Type safe	                    Not type safe
Prevents mixing types	        Can mix with original type
struct Km(i32);	                type Km = i32;*/

//Nver Type(!)

//The Never Type (!) represents a function that never returns.
//It is used when a function:Panics, Loops forever, Exits the program
/*fn crash() -> ! {//never type
    panic!("Program crashed!");
}

fn main() {//main thread panickeddddd
    crash();
}*/

//Dynamically sized typess is atype whose size is known only at runtime.
/*fn main() {
    let s: &str = "Hello Rust";
    println!("{}", s);
}*/

//Sized trait
//Definition

//The Sized trait means the compiler knows the size of a type at compile time.

//Almost every Rust type automatically implements Sized.
/*fn print_value<T: Sized>(value: T) {
    println!("Value received");
}
fn main() {
    print_value(10);
    print_value('A');
}*/

//?Sized means the type may or may not have a known size.It is mainly used with references to dynamically sized types.
/*fn display<T: ?Sized>(value: &T) {
    println!("Reference received");
}
fn main() {
    let text = "Hello";

    display(text);
}*/
