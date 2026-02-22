use std::io;
fn main() {
    let x=5;
    println!("the value of x is : {x}");
   // x=8;// cannot assign twice to immutable variable
   // println!{"The value of x is :{x}"
    let mut y=8;
    println!("the value of y is : {y}");
    y=90;//by using mut we can assign 
    println!{"The value of y is :{y}"};
    //=====constants=====
    const HOURS_IN_MINUTES: u32=60*60;
    println!("the value of hours is :{HOURS_IN_MINUTES}");
  //  const mut HOURS_IN_MINUTES: u32=60*60;//redfination not allowed 
  //  println!("the value of hours is :{HOURS_IN_MINUTES}");
  //====SHADOWING======
    let z=19;
    let z=z+1;
    {
        let z=z*2;
        println!("The value of z in the inner scope is: {z}");
    }
    println!("the value of z is:{z}");
     let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
    
}
//variables are immutables,good for saftey
// by using mut kety word we can re assign the values
//constants are same as variables means immutable but mut is not allowed with constants;constant are always immutable
//the key word is const and the type of value must be annotated;
// constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.
//Rust’s naming convention for constants is to use all uppercase with underscores between words.
//the first variable is shadowed by the second, which means that the second variable is what the compiler will see when you use the name of the variable. 
//Shadowing is different from marking a variable as mut because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword
// when we use the let keyword again, we can change the type of the value but reuse the same name.
//let spaces = "   ";
 //   let spaces = spaces.len();
 //=====DATA TYPES========
 // Rust is a statically typed language, which means that it must know the types of all variables at compile time. 
 //SCALAR TYPES : represent single value
 //four primary scalar types: integers, floating-point numbers, Booleans, and characters.
 //INTEGER TYPE a number without fractional  component
//Signed Integer: number type that can store positive and negative numbers.
//Unsigned Integer: number type that stores only zero and positive numbers.