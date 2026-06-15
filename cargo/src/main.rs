
//cargo has two main profiles dev and release 
//dev when cargo build and release when cargo build --release

//======MAKING USEFUL DOCUMENTATION COMMENTS=======

/*Rust uses documentation comments (///) to document public APIs and generate HTML documentation.
Documentation comments support Markdown formatting and are placed just above the item they describe.
A common structure includes a brief description and an # Examples section with sample code.
Example code inside triple backticks (```) is tested automatically by rustdoc.
Running cargo doc generates HTML documentation using the rustdoc tool.
The generated documentation is stored in the target/doc directory.
Documentation comments help users understand how to use a crate rather than how it is implemented.*/



//cargo doc is the cmnd to generate the documentation
//cargo doc --open for the opening of the html file
//# Examples: Shows how to use the function with sample code.
//# Panics: Describes situations in which the function may panic, helping users avoid unexpected crashes.
//# Errors: Explains possible errors returned by a Result and the conditions that cause them.
//# Safety: Required for unsafe functions; states why the function is unsafe and the invariants callers must maintain.


//Code examples inside documentation comments are called doc tests.

/*
Contained Item Comments (//!) in Rust Notes
//! is used to document the item that contains the comment, rather than the item that follows it.
These comments are commonly placed at the beginning of src/lib.rs or inside a module.
//! comments provide documentation for the entire crate or module.
Unlike ///, there does not need to be any code immediately after a //! comment.
They are useful for explaining the overall purpose and functionality of a crate.
The generated documentation displays this information on the crate's main page.*/

/*
use art::kinds::PrimaryColor;
use art::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}*/
use art::PrimaryColor;
use art::mix;

fn main() {
    // --snip--
}

/*
Create a crates.io account using a GitHub account and generate an API token.
Use cargo login to save the API token locally in ~/.cargo/credentials.toml.
The API token is secret and should never be shared.
Every crate must have a unique name on crates.io.
Required metadata in Cargo.toml includes name, version, description, and license.
Common licenses used in Rust are MIT and Apache-2.0.
Publish a crate using the cargo publish command.
Published versions are permanent and cannot be overwritten or deleted.
To release updates, increase the version number following Semantic Versioning and run cargo publish again.
Faulty versions can be removed from future use with cargo yank --vers <version>, and restored with cargo yank --vers <version> --undo.*/

