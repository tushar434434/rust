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

    // Call add_one_v2 with 5 and print the result
    println!("{}", add_one_v2(5)); // Output: 6

    // Call add_one_v3 with 5 and print the result
    println!("{}", add_one_v3(5)); // Output: 6

    // Call add_one_v4 with 5 and print the result
    println!("{}", add_one_v4(5)); // Output: 6
}


// Regular function with an explicit parameter type and return type
fn add_one_v1(x: u32) -> u32 {
    // The last expression is returned automatically
    x + 1
}