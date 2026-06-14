//============Closures =============
// Closures are anonymous functions that can capture variables from their surrounding scope. They are often used for short-lived operations, such as passing a function as an argument to another function or creating a quick function on the fly.


#[derive(Debug,PartialEq,Copy,Clone)]//The #[derive(Debug, PartialEq, Copy, Clone)] attribute automatically implements the Debug, PartialEq, Copy, and Clone traits for the ShirtColor enum. This allows us to print ShirtColor values using {:?}, compare them for equality, and easily copy and clone them when needed.
enum ShirtColor {// define enum shirtcolor 
    Red,
    Blue,
}

struct Inventory {// define struct inventory
    shirts: Vec<ShirtColor>,// use vector to stor rang
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
        if nred > nblue {  // on the basis of count 
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