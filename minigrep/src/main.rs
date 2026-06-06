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
}