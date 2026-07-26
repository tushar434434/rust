//Patterns in Rust are used to match the structure of data and control program flow. They allow you to extract values, ignore values, or check specific conditions in a clean and readable way.
/*A pattern can contain:
Literals (5, "hello")
Variables (x, name)
Tuples ((x, y))
Structs and Enums
Arrays
Wildcards (_)
Placeholders (..)*/


//Paterns in match 
//match compares a value agints multiple patterns
/*
fn main(){
    let num = 2;
    match num {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Other number"),//_ is a wildcard pattern that matches any value not explicitly listed above
    }
}*/

//paterns in let
//let statement also use patterns
/*
fn main(){
    let (x,y,z) = (1, 2, 3);
    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);   
}
*/

//Patterns in if let 

//use if let when you only care about one matching case
/*
fn main(){
    let num =Some(5);
    if let Some(x) = num {
        println!("number is {}",x);
    }
    else {
        println!("number is None");
    }
}*/
/*
fn main(){
    let fav_color : Option<&str> =None;
    let is_tuesday =falseS;
    let age : Result<u8,_> =  "34".parse();
    if let Some(color)= fav_color{
        println!("your fav color is {}",color);
    }
    else if is_tuesday{
        println!("Tuesday is green day");
    } 
    else if let Ok(age) = age {
        if age > 30 {
            println!("purple");
        }
        else {
            println!("orange");
        }
    }
    else {
        println!("no color");
    }
}*/

//Patterns in while let 
//loop while a pattern matches
/*
fn main(){
let mut stack =vec![1,2,3];
while let Some(top) = stack.pop(){
    println!("{}",top);
}}*/

//patterns in for 
//destructure values while looping
/*
fn main(){
    let fruits = vec!["apple","banana","cherry"];
    for (index,value) in fruits.iter().enumerate(){
        println!("{}: {}",index,value);
    }
}*/

//pattren in functions

//function parameters can also use patterns to destructure values passed to the function
/*
fn print(&(x,y): &(i32,i32)){
    println!("x: {}, y: {}",x,y);
}
fn main(){
    let point = (10,20);
    print(&point);
}*/
//Patterns are a special syntax used to match the structure of data, extract values, ignore values, and control program flow.
//_ matches any value but ignores it. It is commonly used as a catch-all case in match.
//if let is shorter and more readable when you only need to handle one matching pattern.



//=======Refutability of patterns ========

//patterns are classified as refutable or irrefutable. An irrefutable pattern will always match, while a refutable pattern may not match. For example, a variable binding is an irrefutable pattern, while a match arm with a specific value is a refutable pattern.
//irrefutable patterns can never fail to match

//a refutable pattern might fail for some values
/*
Place	                            Irrefutable	            Refutable
let             	                ✅ Required	           ❌ Not allowed
Function Parameters	                ✅ Required	           ❌ Not allowed
for loop	                        ✅ Required	           ❌ Not allowed
if let	                            ✅ Allowed	           ✅ Preferred
while let	                        ✅ Allowed	           ✅ Preferred
let...else	                        ❌ Warning if irrefutable	✅ Preferred
match arms                      	Mostly refutable	    Last arm can be _