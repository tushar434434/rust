//============Closures =============
// Closures are anonymous functions that can capture variables from their surrounding scope. They are often used for short-lived operations, such as passing a function as an argument to another function or creating a quick function on the fly.

/*
#[derive(Debug,PartialEq,Copy,Clone)]//The #[derive(Debug, PartialEq, Copy, Clone)] attribute automatically implements the Debug, PartialEq, Copy, and Clone traits for the ShirtColor enum. This allows us to print ShirtColor values using {:?}, compare them for equality, and easily copy and clone them when needed.
enum ShirtColor {// define enum shirtcolor 
    Red,
    Blue,
}

struct Inventory {// define struct inventory
    shirts: Vec<ShirtColor>,// use vector to store rang
}

impl Inventory { // implement inventory
    fn giveaway(&self, user_prefernce: Option<ShirtColor>) ->ShirtColor {// define function giveaway that takes an optional shirt color preference and returns a shirt color
        user_prefernce.unwrap_or_else(|| self.most_stocked())// if the user has a preference, return it. Otherwise, call the most_stocked method to determine which shirt color to give away.
    }

    fn most_stocked(&self)->ShirtColor {

        let mut nred =0;
        let mut nblue =0;
        for color in &self.shirts {
            match color {//simply count kra hai
                ShirtColor::Red => nred +=1,
                ShirtColor::Blue => nblue +=1,
            }
        }
        if nred > nblue {  // on the basis of count compare kara hai
            ShirtColor::Red
        } else {
            ShirtColor::Blue    
        }
    }
}

fn main(){
    let store =Inventory { // create an instance of Inventory with a list of shirts
        shirts: vec![ShirtColor::Red, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);// define a user preference for a red shirt
    let giveaway1 = store.giveaway(user_pref1);// call the giveaway method with the user's preference and store the result
    println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);
    let user_pref2 = None;// define a user preference for no particular shirt color
    let giveaway2 = store.giveaway(user_pref2);// call the giveaway method without a user preference and store the result
    println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);
}

*/


//Inferring and annotating closure types
/*
use std::thread;
use std::time::Duration;

fn main() {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    println!("{}", expensive_closure(5));
}*/

/*use std::thread;
use std::time::Duration;
// yha se closure aur use anotation ke type ke baare me hai
fn main() {
    // Call the regular function add_one_v1 with the argument 5
    // The function returns 6, which is printed to the console
    println!("{}", add_one_v1(5)); 

    // Define a closure with an explicit parameter type (u32)
    // and an explicit return type (u32)
    let add_one_v2 = |x: u32| -> u32 {
        x + 1 // Return x + 1
    };

    // Define a closure with inferred parameter and return types
    // Rust determines the types based on how the closure is used
    let add_one_v3 = |x| {
        x + 1 // Return x + 1
    };
    // Define the same closure in its shortest form
    // Braces are optional because the body contains only one expression
    let add_one_v4 = |x| x + 1;
    
    println!("{}", add_one_v2(5)); // Output: 6
    println!("{}", add_one_v3(5)); // Output: 6
    println!("{}", add_one_v4(5)); // Output: 6
}

// Regular function with an explicit parameter type and return type
fn add_one_v1(x: u32) -> u32 {
    // The last expression is returned automatically
    x + 1
}
*/
/*

fn main(){
        let example_closure = |x| x;

    let s = example_closure(String::from("hello"));// The closure example_closure is called with a String argument, so Rust infers that the type of x is String. The closure simply returns the value of x, which is "hello". Therefore, s will be assigned the value "hello".
    let n = example_closure(5);
    println!("s: {}, n: {}", s, n);
    // This code will not compile because the closure example_closure is being used with two different types (String and i32). Rust cannot infer a single type for the parameter x, leading to a type inference error. To fix this, you would need to specify the type of x in the closure definition or ensure that the closure is only used with one type.
    //error dega kyuki s ne type phle string se infer kiya hai aur n ne integer se infer kiya hai, toh compiler ko pata nahi chalega ki x ka type kya hai. To fix this, you can specify the type of x in the closure definition like this:
    
}*/

//capturing the refrerence or moving ownership
/*
fn main(){
    let list = vec![1,2,3];// The variable list is defined as a vector containing the integers 1, 2, and 3. This vector is stored on the heap, and list is a reference to that heap-allocated memory.
    println!("before defiening the closure : {list:?}");

    let only_borrow = || println!("from closure: {list:?}");// The closure only_borrow captures the variable list by reference. This means that the closure borrows list and can read its value, but it does not take ownership of it. As a result, we can still use list after defining the closure without any issues.
    println!("before calling the closure : {list:?}");
    only_borrow();// When we call the closure only_borrow(), it prints the value of list, which is still accessible because the closure only borrows it. The output will show the contents of list as [1, 2, 3].
    println!("after calling the closure : {list:?}");
}
/* ye output dega:
before defiening the closure : [1, 2, 3]
before calling the closure : [1, 2, 3]
from closure: [1, 2, 3]
after calling the closure : [1, 2, 3]*/
*/
/*
fn main(){
    let mut list =vec![1,2,3];// The variable list is defined as a mutable vector containing the integers 1, 2, and 3. This vector is stored on the heap, and list is a mutable reference to that heap-allocated memory.
    println!("before defining the closure: {list:?}");
    let mut borrows_mutable = || list.push(7);// The closure borrows_mutable captures the variable list by mutable reference. This means that the closure can modify the contents of list. However, because it takes a mutable borrow, we cannot use list elsewhere while the closure is in scope.
  // println!("before calling the closure: {list:?}");// When we try to print list before calling the closure, it will cause a compile-time error because list is already borrowed mutably by the closure. Rust's ownership rules prevent us from using list while it is borrowed mutably.
    borrows_mutable();// When we call the closure borrows_mutable(), it modifies the
    println!("after calling the closure: {list:?}");// After calling the closure, we can print list again. The closure has modified list by adding the value 7 to it, so the output will show the updated contents of list as [1, 2, 3, 7].
}
*/

use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}")).join().unwrap();
// The closure is defined with the move keyword, which transfers ownership of list into the closure. This allows the closure to be executed in a separate thread without any borrowing issues. The thread prints the contents of list, which is now owned by the closure.

    // println!("After defining closure: {list:?}"); // This line would cause a compile-time error because list has been moved into the closure and is no longer accessible in the main thread.
}

