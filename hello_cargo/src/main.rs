/*fn main() {
    println!("Hello, world!");
}*/
//rustup update
//cargo is rust build system and package manager.
/*cargo build -- compile rust project  
Converts .rs files into an executable.
Does NOT run the program.
Creates a target/ folder.
Default: debug mode.
//cargo run 
Compiles and runs the project.
If already built → only runs.
If changes detected → rebuilds then runs.*/
//cargo check --- quickly checks your code to make sure it compiles but doesn’t produce an executable:
//cago check is faster then cargo build bcs it skips the step of producing an executable.
//Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.
//cargo build --release to compile it with optimizations. This command will create an executable in target/release instead of target/debug
//---------NUMBER GUESSING GAME-------
use std::cmp::Ordering;//The Ordering type is another enum and has the variants Less, Greater, and Equal
use std::io;//input output library
use rand::Rng;//The Rng trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods.
fn main(){
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);//start..=end and is inclusive on the lower and upper bounds,
    loop{
    println!("Please input yor guess.");
    let mut guess=String::new();
    io::stdin().read_line(&mut guess).expect("Failed read line");
   // let guess: u32 = guess.trim().parse().expect("Please type a number!");//without this typecastring problem :The reason for the error is that Rust cannot compare a string and a number type.
   let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) =>{ 
                println!("Please enter a valid number!");
                continue;}
        };//skiping the invalid input by the user
    //before it ,it run only one time .
    //Rust allows us to shadow the previous value of guess with a new one.
    // (On Windows, pressing enter results in a carriage return and a newline, \r\n.) The trim method eliminates \n or \r\n, resulting in just 5.
    println!("You guessed:{guess}");
    match guess.cmp(&secret_number){//cmp is for compare 
        // The match expression gets the Ordering::Greater value and starts checking each arm’s pattern.
        Ordering::Less=>println!("Too small!"),
        Ordering::Greater=>println!("Too big!"),//The match expression ends after the first successful match,
        Ordering::Equal=>{
            println!("You win!");
            break;
            }
        }
    }
}
// In Rust, variables are immutable by default
//using mut make them mutable
//String::new, a function that returns a new instance of a String
// An associated function is a function that’s implemented on a type
// a crate is a collection of Rust source code files. 
//Rust does NOT include random functions in the standard library.
//use the external crate rand
//When we include an external dependency, Cargo fetches the latest versions of everything that dependency needs from the registry, which is a copy of data from Crates.io. Crates.io is where people in the Rust ecosystem post their open source Rust projects for others to use.
//cargo lock=What versions are actually used (exact)
//The parse method on strings converts a string to another type
//using let guess: u32. The colon (:) after guess tells Rust we’ll annotate the variable’s type.
