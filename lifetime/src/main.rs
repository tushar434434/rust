//every referece has  a lifetime, which is the scope for which that reference is valid.
//Dangling references: a reference that points to data that has been dropped. Rust prevents this by ensuring that the lifetime of the reference is always less than the lifetime of the data it points to.
//Lifetimes are denoted with an apostrophe followed by a name, like 'a. They are used to specify the scope of references in functions and structs.//Lifetimes are a compile-time feature that ensures memory safety without needing a garbage collector. They allow Rust to determine how long references should be valid and prevent dangling references.//
//genric lifetimes in functions
/*
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {//'a is the lifetime parameter, it can be any name but 'a is convention
    if x.len() > y.len() {
        x//returning x or y is fine because they have the same lifetime 'a
    } else {
        y//returning x or y is fine because they have the same lifetime 'a
    }
}*/
//in function signatures, we can specify that the lifetimes of the parameters and return value are related. In this example, the function longest takes two string slices with the same lifetime 'a and returns a string slice with the same lifetime 'a. This means that the returned reference will be valid as long as both input references are valid.
//in structs, we can also specify lifetimes for references. 
//lifetime elision rules: Rust has three rules for inferring lifetimes when they are not explicitly specified. These rules allow Rust to determine the lifetimes of references based on the structure of the code. The rules are:
//1. Each parameter that is a reference gets its own lifetime parameter.            
//2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
//3. If there are multiple input lifetime parameters, but one of them is &self or
//&mut self, the lifetime of self is assigned to all output lifetime parameters.
//these rules allow us to write code without having to specify lifetimes in many cases, while

//in method definitions, the first parameter is always &self or &mut self, so the lifetime of self is automatically assigned to the output lifetime parameters. This means that we can often omit lifetimes in method definitions and let Rust infer them based on these rules.

//the static lifetime: 'static is a special lifetime that denotes that the reference can live for the entire duration of the program. It is often used for string literals, which are stored in the binary and have a fixed location in memory. A reference with a 'static lifetime can be stored in a variable and used throughout the program without worrying about it being dropped.
//example: let s: &'static str = "I have a static lifetime.";
//In this example, the string literal "I have a static lifetime." has a 'static

//genric type parameters trait bounds and lifetimes can be combined to create more flexible and reusable code. For example, we can define a function that takes a reference to any type that implements a certain trait, and also specify the lifetime of that reference. This allows us to write functions that can work with a wide range of types while still ensuring memory safety through lifetimes.
/*
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(// generic type parameter T and lifetime parameter 'a
    x: &'a str,//the lifetime of x is 'a
    y: &'a str,//the lifetime of y is 'a
    ann: T,
) -> &'a str//the return type has the same lifetime 'a as the input parameters
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}*/

//revise lifetime rules: The lifetime of a reference must always be less than or equal to the lifetime of the data it points to. This means that a reference cannot outlive the data it refers to, preventing dangling references and ensuring memory safety.