/*use std::env;//The env module provides functions for working with environment variables and command-line arguments. In this case, we are using it to access the command-line arguments passed to the program.

fn main() {
    let args: Vec<String> = env::args().collect();//env::args() returns an iterator of the command-line arguments, and collect() gathers them into a vector of strings. The first argument (args[0]) is the name of the program itself, and subsequent arguments are the ones passed by the user.
    dbg!(args);//dbg! is a macro that prints the value of the expression passed to it, along with the file and line number where it was called. This is useful for debugging purposes, as it allows you to see the contents of the args vector when the program is run.
}
//the args function and invalid unicode in the command line will cause an error. To handle this, we can use the std::env::args_os function, which returns an iterator of OsString values that can represent any valid Unicode string, including those that may not be valid UTF-8.
//the output of the above code with the arguments "hello world" would be something like:
//[src\main.rs:5] args = [
//    "target\\debug\\minigrep.exe",
//    "hello",      
//    "world"
//] 
//The first element is the name of the program, and the subsequent elements are the command-line arguments passed by the user.
//saving the arguiment values in variables
*/
/*
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];//The first argument (args[0]) is the name of the program itself, so we access the second argument (args[1]) to get the query string that the user wants to search for.
    let file_path = &args[2];//The third argument (args[2]) is the file path that the user wants to search in. We store it in the file_path variable for later use.

    println!("Searching for {query}");//This line prints a message to the console indicating what query we are searching for. The {query} syntax is a placeholder that will be replaced with the actual value of the query variable when the program runs.
    println!("In file {file_path}");//This line prints a message to the console indicating which file we are searching in. Similar to the previous line, the {file_path} syntax is a placeholder that will be replaced with the actual value of the file_path variable when the program runs.
}*/

/*
use std::env;
use std::fs;//The fs module provides functions for working with the file system, such as reading and writing files. In this case, we are using it to read the contents of a file specified by the user.

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}*/

/*
Problems in the Original Code
==>main() was doing too many things:
=>Parsing arguments
=>Reading files
=>Running search logic
=>Handling errors
=>Configuration values (query, file_path) were scattered.
=>Error messages were not user-friendly.
=>panic! was being used for user mistakes.*/
//Separting the code into multiple functions to improve readability and maintainability.
/*use std::env;
use std::process;
use std::error::Error;
use std::fs;

use minigrep::search;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}*/
use std::env;      // Provides access to command-line arguments and environment variables
use std::process;  // Allows us to terminate the program with an exit code
use std::error::Error; // Trait for handling different kinds of errors
use std::fs;       // Used to read files

// Import both search functions from the library
use minigrep::{search, search_case_insensitive};

// Struct to store configuration values
struct Config {
    query: String,      // The text we want to search for
    file_path: String,  // The file to search in
    ignore_case: bool,  // Whether the search should ignore case
}

impl Config {
    // Creates a Config instance from command-line arguments
    fn build(args: &[String]) -> Result<Config, &'static str> {

        // Ensure the user provides enough arguments
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // Clone the query and file path from the arguments
        let query = args[1].clone();
        let file_path = args[2].clone();

        // Check if the IGNORE_CASE environment variable exists
        // Returns true if it is set, false otherwise
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        // Return the Config instance
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// Runs the application
fn run(config: Config) -> Result<(), Box<dyn Error>> {

    // Read the contents of the file
    let contents = fs::read_to_string(config.file_path)?;

    // Decide which search function to use
    let results = if config.ignore_case {

        // Perform case-insensitive search
        search_case_insensitive(&config.query, &contents)

    } else {

        // Perform normal case-sensitive search
        search(&config.query, &contents)
    };

    // Print each matching line
    for line in results {
        println!("{line}");
    }

    Ok(())
}

fn main() {

    // Collect command-line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Build the configuration or exit with an error
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Run the program and handle any errors
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
//redirecting  errors to standard error
// Redirecting Errors to Standard Error
// 1. Use println!() for normal program output (stdout).
// 2. Use eprintln!() for error messages so they are sent to stderr.
// 3. Separating stdout and stderr allows successful output to be redirected to files while keeping errors visible on the terminal.



