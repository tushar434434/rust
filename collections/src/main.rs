//Common collections
//storing list of values with vector
//To create a new, empty vector, we call the Vec::new function
// let v: Vec<i32> = Vec::new();
//store same type of values with vector
// let v = vec![1, 2, 3]; default i32 type
//Vectors are implemented using generics, so we can store any type of data in a vector, as long as all the data is of the same type. We can specify the type of data we want to store in a vector using angle brackets (<>). For example, if we want to store a vector of strings, we can declare it like this:
//==updating a vecto
/*
 let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);*/
//==reading elements of a vector
/*
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
     for i in &v {
        print!("{i} ");}
        println!();
    let mut m = vec![100, 32, 57];
    for i in &mut m{
        *i+=10;//dereference to get the value and add 10 to it
    }

    println!("Modified vector: {m:?}");
}*/
//When we use the get method with the index passed as an argument, we get an Option<&T> that we can use with match
/*Using Indexing ([])
Direct access
Returns: &i32
⚠️ Panics if index is out of bounds*/
/*Using .get() method
Safe access
Returns: Option<&i32>
No panics, but we need to handle the None case*/
//v[2] → fast but unsafe (can crash)
//v.get(2) → safe (returns Option)


// using vectors to store values of different types with enum
/*
fn main() {
    enum SpreadsheetCell {
        Int(i32),//variant that holds an integer value
        Float(f64),//variant that holds a floating-point value
        Text(String),  //variant that holds a string value
    }

    let row = vec![
        SpreadsheetCell::Int(3),//create an instance of the Int variant with the value 3
        SpreadsheetCell::Text(String::from("blue")),//create an instance of the Text variant with the value "blue"
        SpreadsheetCell::Float(10.12),//create an instance of the Float variant with the value 10.12
    ];

    for cell in row {
        match cell {
            SpreadsheetCell::Int(i) => println!("Int: {i}"),
            SpreadsheetCell::Float(f) => println!("Float: {f}"),
            SpreadsheetCell::Text(t) => println!("Text: {t}"),
        }
    }
}
*/
//Droping a vector
//When a vector goes out of scope, Rust automatically calls the drop function to free the memory that the vector was using. This is because Rust has a feature called ownership, which ensures that memory is automatically managed and prevents memory leaks. When a vector is dropped, all of its elements are also dropped, and the memory they were using is freed. This means that you don't have to worry about manually freeing memory when you're done with a vector, as Rust takes care of it for you.


//========Strings========
//A String is a growable, heap-allocated data structure that is used to store and
//creating a new string
/*let mut s = String::new();
let data = "initial contents";*/
/*
fn main() {
    let data = "initial contents";

    // Convert &str to String
    let mut s1 = data.to_string();

    // Convert directly from string literal
    let s2 = "initial contents".to_string();

    println!("s1: {s1}");
    println!("s2: {s2}");
    s1.push_str(" more data"); // Append a string slice to a String
    println!("s1 after push_str: {s1}");
    let s3 =s1 + &s2; // Concatenate s1 and s2, resulting in a new String s3. Note that s1 is moved and can no longer be used after this point.&s2 is a reference to s2, so it can still be used after the concatenation.
    println!("s3: {s3}");

}*/
/*
//Using the + operator to concatenate strings
fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;// Concatenate s1, s2, and s3 with hyphens in between. Note that s1 is moved and can no longer be used after this point. &s2 and &s3 are references to s2 and s3, so they can still be used after the concatenation.
//add method can only concatenate a string slice (&str) to a String, so we need to use &s2 and &s3 to pass references to the strings instead of moving them. The + operator is overloaded to perform string concatenation when used with String and &str types.
    println!("Result: {s}");
    // println!("{s1}"); ❌ ERROR (s1 moved)
    println!("s2: {s2}");
    println!("s3: {s3}");
}*/
//Using the format! macro to concatenate strings
/*
fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");// Concatenate s1, s2, and s3 with hyphens in between using the format! macro. This does not take ownership of s1, s2, or s3, so they can still be used after this point. The format! macro is a more flexible way to concatenate strings because it allows you to include multiple variables and format them in various ways without taking ownership of the original strings.

    println!("Result: {s}");

    // All still usable
    println!("s1: {s1}");
    println!("s2: {s2}");
    println!("s3: {s3}");
}*/
//The format! macro is a more flexible way to concatenate strings because it allows you to include multiple variables and format them in various ways without taking ownership of the original strings. It also provides better readability and maintainability compared to using the + operator for string concatenation, especially when dealing with multiple strings or complex formatting requirements.


//Indexing into strings
//In Rust, you cannot directly index into a String using the [] syntax because a String is
//a collection of bytes, and not all byte sequences are valid UTF-8 characters. This means that indexing into a String could potentially lead to invalid character boundaries, which is why Rust does not allow it. Instead, you can use methods like chars() or bytes() to iterate over the characters or bytes of a String safely. If you need to access a specific character, you can convert the String into a vector of characters using the chars() method and then index into that vector. However, keep in mind that this approach may not be efficient for large strings, as it involves creating an intermediate vector of characters.
//===internal representation of a string
//A String is a wrapper over a Vec<u8>. It is a growable, heap-allocated data structure that stores a sequence of bytes. The String type ensures that the bytes it contains are valid UTF-8, which allows it to represent text in a way that can be easily manipulated and displayed. When you create a String, it allocates memory on the heap to store the bytes, and it provides methods for adding, removing, and modifying the contents of the string. The internal representation of a String as a Vec<u8> allows it to efficiently manage memory and provide fast access to its contents while ensuring that the data is valid UTF-8.
//number of bytes it takes to encode “Здравствуйте” in UTF-8, because each Unicode scalar value in that string takes 2 bytes of storage.
//Therefore, an index into the string’s bytes will not always correlate to a valid Unicode scalar value.

//====Slicing Strings======
//A string slice is a reference to a contiguous sequence of characters in a String. It is represented by the &str type and is used to borrow a portion of a String without taking ownership of it. A string slice can be created by using the slicing syntax, which involves specifying a range of indices within the String. For example, if you have a String called s, you can create a string slice that references the first five characters of s using the syntax &s[0..5]. This creates a string slice that points to the characters at indices 0 through 4 of the original String. String slices are useful for working with substrings and for passing references to parts of a String without needing to create new String instances.


//iterating over the characters of a string
/*for b in "Зд".bytes() {
    println!("{b}");
}*/
/*
208
151
208
180*/
/*
for c in "Зд".chars() {
    println!("{c}");
}
*/
/*З
д
The chars() method iterates over the Unicode scalar values (characters) in the string, while
*/


//========Hash Maps========
/*
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);// Insert a key-value pair into the scores HashMap, where the key is "Blue" and the value is 10
    scores.insert(String::from("Yellow"), 50);

    // Access values
    for (key, value) in &scores {// Iterate over the key-value pairs in the scores HashMap
        println!("{key}: {value}");
    }

    // Get a value
    let team = String::from("Blue");
    match scores.get(&team) {// Attempt to retrieve the score for the "Blue" team using the get method, which returns an Option<&V>
        Some(score) => println!("Score of {team}: {score}"),// If the team is found, print its score
        None => println!("Team not found"),//   If the team is not found, print a message indicating that the team was not found
    }
}*/

//overwriting a value
/*    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");
    // Output: {"Blue": 25} (the value for "Blue" is overwritten to 25)*/
    
    
    //Hash maps have a special API for this called entry that takes the key you want to check as a parameter. The return value of the entry method is an enum called Entry that represents a value that might or might not exist
      /*  use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");*/
    // Output: {"Blue": 10, "Yellow": 50} (the value for "Blue" remains 10, while "Yellow" is inserted with a value of 50)
use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {// Split the text into words using whitespace as a delimiter and iterate over each word
        let count = map.entry(word).or_insert(0);// For each word, use the entry method to get a mutable reference to the count of that word in the map. If the word is not already in the map, or_insert(0) will insert it with an initial count of 0 and return a mutable reference to that count.
        *count += 1;// Increment the count for the word by dereferencing the mutable reference and adding 1 to it
    }

    println!("{map:?}");
}