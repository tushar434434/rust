//Enums (Enumerations) allow a value to be one of several possible types (variants).
//Variants are accessed using :: (e.g., IpAddrKind::V4).
//the option enum   
//rust dosent have null values but we can use the option enum to represent a value that can be either something or nothing
/// An enum that represents a value that may or may not exist.
/*enum Option<T> {
    None,
    Some(T),
}*/
  /*  let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;*/
//this code will give an error because we cannot add an i8 and an Option<i8> directly. We need to handle the Option type properly, for example by using pattern matching or the unwrap method to extract the value from the Option before performing the addition.
//THE match control flow construct
//extremely powerful control flow construct called match that allows you to compare a value against a series of patterns and then execute code based on which pattern matches.
//match arms consist of a pattern to match and the code to run if the value matches that pattern. Patterns can be literals, variables, wildcards, or even more complex structures like tuples and enums. The match expression evaluates the value against each pattern in order, and executes the code for the first pattern that matches. If no patterns match, it will result in a compile-time error unless a catch-all pattern (using _) is provided.
//Debug is a trait that allows a type to be printed for debugging.
//#[derive(Debug)] automatically generates the implementation.
//Without Debug, you can’t print structs/enums using println!("{:?}", value).
//Quarter(UsState) = enum variant with data
//state = value extracted from that variant
//This is called pattern matching with binding
  /*  fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);//give error bcs of ownershi
    let none = plus_one(None);
    println!("five: {:?}, six: {:?}, none: {:?}", five, six, none);*/

/*fn plus_one(x: &Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(&five);
    let none = plus_one(&None);

    println!("five: {:?}, six: {:?}, none: {:?}", five, six, none);
}*/
//Catch-All Patterns & _ in Rust (Summary)
//match must be exhaustive → all possible values must be handled.
//Catch-all using variable
/*
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}*/
//other = matches all remaining values
//Value is captured and used
//Catch-all using _ (placeholder)
//_ = matches everything else
//Value is ignored (not stored)
//===========Concise Control Flow with if let and let...else=====
