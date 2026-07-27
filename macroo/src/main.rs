/*A macro is a feature that writes Rust code during compile time (metaprogramming). Macros generate code before the compiler compiles the program.
Unlike functions, macros:
Can accept a variable number of arguments.
Work at compile time.
Can generate Rust code.*/

/*Difference Between Macros and Functions
Function	                        Macro
Runs at runtime	                    Expands at compile time
Fixed number of parameters	        Variable number of parameters
Cannot generate code	            Can generate Rust code
Defined using fn	                Defined using macro_rules! or procedural macros
*/
//vec! generates code to create a vector.

//==Declerative macroooosssssss========

//A Declarative Macro (also called a macro_rules! macro) matches patterns and generates Rust code based on those patterns.
/*
macro_rules! square {
    ($x:expr) => {//In Rust macros, $x is a macro variable (or metavariable).
        println!("{}", $x * $x);
    };
}
fn main() {
    square!(6);
}*/
//vec! is a built-in declarative macro that creates a vector with any number of elements.

//procedural macrosssss======
/*A Procedural Macro receives Rust code as input (TokenStream), processes it, and generates new Rust code.

There are three types:

Custom Derive Macros
Attribute-like Macros
Function-like Macros*/


//custom derive macro ==A Custom Derive Macro automatically generates trait implementations using the #[derive(...)] attribute.
//An Attribute-like Macro creates custom attributes that can be attached to functions, structs, modules, and more.
/*
#[route(GET, "/")]//#[route(...)] is an attribute-like macro (commonly used in web frameworks like Rocket or Actix Web).
fn index() {
    println!("Home Page");
}*/
//A Function-like Macro looks like a function call but operates on Rust tokens at compile time.
//eg sql!(SELECT * FROM users);//These are macros, not regular functions.

//stringify! macro ==>converts Rust code into a string without evaluating it.
/*fn main() {
    println!("{}", stringify!(10 + 20));//output will be 10+20 not 30
}*/
//println! macro prints formatted text to the console

//format macro! creates a formated string instead of printing it.
/*
fn main() {
    let language = "Rust";

    let message = format!("Hello {}", language);

    println!("{}", message);
}*/

//Macros expand at compile time, can accept a variable number of arguments, and generate code. Functions execute at runtime with fixed parameter types.

/*procedural macros?
Custom derive macros (#[derive(...)])
Attribute-like macros (#[route(...)])
Function-like macros (sql!(...))*/
//macro_rules! is Rust's declarative macro system that generates code by matching patterns.
//Use macros when you need compile-time code generation, flexible argument lists, or functionality that cannot be achieved with regular functions.