/// A crate = smallest unit of code compilation in Rust
/// It’s basically a package/module of code
//A package is a bundle of one or more crates that provides a set of functionality. A package contains a Cargo.toml file that describes how to build those crates
/*
Crate root: Entry point is src/main.rs (binary) or src/lib.rs (library).
Modules (mod): Declared in crate root → compiler looks:
Inline { }
src/module.rs
src/module/mod.rs
Submodules: Declared inside modules → compiler looks:
Inline { }
src/parent/submodule.rs
src/parent/submodule/mod.rs
Paths: Access code using full path
→ crate::garden::vegetables::Asparagus
Privacy:
Default = private
Use pub to make modules/items public
use keyword:
Creates shortcuts
→ use crate::garden::vegetables::Asparagus;
Then just use Asparagus instead of full path*/

/*A crate is the smallest compilation unit in Rust.
Two types:
Binary crate → has main() (executable)
Library crate → no main(), used as dependency
Entry point:
src/main.rs (binary)
src/lib.rs (library)*/
/*Module (mod)
Used to organize code into namespaces
Declared using:*/
/*pub (Visibility)
By default, everything is private
pub makes items accessible outside module*/
/*use Keyword
Brings paths into scope (shortcut)*/

//=====Paths for Referring to an Item in the Module Tree===
/*
Absolute Path
Starts from the crate root
Always begins with:
crate::
or external crate name*/
/*
Relative Path
Starts from the current module
Uses:
self → current module
super → parent module*/

//=====Starting Relative Paths with super====
//super refers to the parent module of the current module


//Making Structs and Enums Public
//By default, all struct fields and enum variants are private, even if the struct or enum itself is public. To make them accessible from outside the module, you need to declare each field or variant as public using the pub keyword.
/*
mod back_of_house {// root module
    pub struct Breakfast {// public struct
        pub toast: String,// public field
        seasonal_fruit: String,  // private field
    }

    impl Breakfast {// implementation block for Breakfast
        pub fn summer(toast: &str) -> Breakfast {// public associated function (constructor)
            Breakfast {
                toast: String::from(toast),// initialize toast field with provided argument
                seasonal_fruit: String::from("peaches"),// initialize seasonal_fruit field with "peaches"
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");

}
//In contrast, if we make an enum public, all of its variants are then public
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}*/


//Bringing Paths into Scope with the use Keyword

//Adding use and a path in a scope is similar to creating a symbolic link in the filesystem.
//using extern crate to bring an external crate into scope
//extern crate rand; // This is no longer needed in Rust 2018 edition and later


//Separating Modules into Different Files