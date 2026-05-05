//Errors
//error are of types recoverable and unrecoverable
//unrecoverable → panic! macro
//recoverable → Result<T, E> enum
//Rust groups errors into two major categories: recoverable and unrecoverable errors. For a recoverable error, such as a file not found error, we most likely just want to report the problem to the user and retry the operation. Unrecoverable errors are always symptoms of bugs, such as trying to access a location beyond the end of an array, and so we want to immediately stop the program.


//Unrecoverable Errors with panic!
//occur:by taking an action that causes our code to panic (such as accessing an array past the end) or by explicitly calling the panic! macro.


//Recoverable Errors with Result<T, E>
//Rust has a type called Result<T, E> that is intended for use in the return
//The T and E are generic type parameters
//T stands for the type of the value that will be returned in a success case, and E stands for the type of the error that will be returned in a failure case. By convention, the Ok variant is used to indicate success and contains a value, and the Err variant is used to indicate failure and contains an error value.
//handling recoverable errors through matching on the Result<T, E> enum and using the panic! macro for unrecoverable errors. Rust also has a ? operator that can be used to propagate errors in a concise way, which is often used in functions that return a Result<T, E> type.
/*
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}*///paniced because hello.txt does not exist
/*
use std::fs::File;//use std::io::ErrorKind;//to handle different kinds of errors
use std::io::ErrorKind;

fn main() {
    let _file = match File::open("hello.txt") {
        Ok(file) => file,//if the file is opened successfully, it will be assigned to _file
        Err(error) => match error.kind() {//if there is an error, we match on the kind of error
            ErrorKind::NotFound => File::create("hello.txt")//if the error is NotFound, we try to create the file
                .expect("Problem creating the file"),//if there is an error creating the file, we panic with a message
            _ => panic!("Problem opening the file: {error:?}"),//if the error is not NotFound, we panic with a message
        },
    };

    println!("File handled successfully");
}*/

/*
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let _file = File::open("hello.txt").unwrap_or_else(|error| {//unwrap_or_else is a method that takes a closure as an argument and calls the closure if the Result is an Err variant. The closure takes the error as an argument and returns a value that will be used as the new value of the Result.
        if error.kind() == ErrorKind::NotFound {//if the error is NotFound, we try to create the file
            File::create("hello.txt")// if there is an error creating the file, we panic with a message
                .unwrap_or_else(|e| panic!("Problem creating file: {e:?}"))//  if there is an error creating the file, we panic with a message
        } else {
            panic!("Problem opening file: {error:?}");
        }
    });

    println!("File ready");
}*/


 //expect is a method that is similar to unwrap but allows us to specify the panic error message. The expect method is often used when you have a reasonable expectation of success and want to provide a clear error message if that expectation is not met. For example, when you are opening a file that you expect to be present, you might use expect to provide a message that indicates the file was not found if the operation fails.
  /*  use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}*/

////=======propagating Errors ==========
//propagating errors is when you want to return an error from a function to the calling code rather than handling it within the function itself. In Rust, you can propagate errors using the Result<T, E> type and the ? operator. The ? operator allows you to return an error from a function if an operation that returns a Result<T, E> fails, without having to write explicit match statements to handle the error. When you use the ? operator, if the Result is an Err variant, the error will be returned from the function immediately. If the Result is an Ok variant, the value will be unwrapped and returned from the expression.


//=======Using the ? Operator for Concise Error Propagation ==========
/*use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;//The ? operator is used here to propagate any error that occurs when trying to open the file. If the file cannot be opened, the error will be returned from the function immediately.
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
*
