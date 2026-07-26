//Rust normally guarantees memory safety at compile time. However, some low-level tasks (like interacting with C, hardware, or raw memory) require operations that the compiler can't fully verify.
//For these cases, Rust provides Unsafe Rust.
/*
The 5 Unsafe Superpowers:
Dereference raw pointers.
Call unsafe functions or methods.
Access or modify mutable static variables.
Implement unsafe traits.
Access union fields.*/

//Raw pointers are similar to refrences but dont follow rust borrowing rules.
//*const T imutable raw pointer to T
//*mut T mutable raw pointer to T


//creating raw pointers
/*
fn main(){
    let mut num=10;
    let r1 = &raw const num;
    let r2 = &raw mut num;
    unsafe{
    println!("r1 is: {:?}",r1);
    println!("r2 is: {:?}",r2);}

}*/

//Calling unsafe functions
/*
unsafe fn danger(){
    println!("unsafe hai bhai yeeee");
}
fn main(){
    unsafe{
        danger();
    }
}*/


//Safe abstraction over unsafe code 

//Rusts standard library often hides unsafe code behind safe apis
/*
fn main(){
    let mut num = vec![1,2,3,4,5];
    let (left,right) =num.split_at_mut(2);
    println!("{:?}",left);
    println!("{:?}",right);
}*/

//Calling c functions (ffi)
//rust can call c functions using extern
/*
unsafe extern "C" {
    safe fn abs(input: i32) ->i32;
}
fn main(){
    println!("{}", abs(-20));
}*/

//Static variables
//imutable static variables are safe
/*
static HELLO: &str = "Hello Rust";//hello should be capital otherwise warning degaaaaaa
fn main(){
    println!("{}",HELLO);
}*/

//mutable static variables
/*
static mut COUNTER: u32=0;//error
fn main(){
    unsafe{
        COUNTER +=1;
        println!("{}",COUNTER);
    }
}*/
//unsafe traits
//sometimes rust cannot verify trait safety;
/*
unsafe trait Demo {}
unsafe impl Demo for i32 {}
fn main(){
    println!("unsafe traits");
}*/

//Miri is a rust tool that detects undefined behaviour at runtime
/*Miri helps find:
Invalid pointers
Undefined behavior
Memory errors
Dangling pointers*/