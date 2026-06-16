//Pointer==contains address in memory
//smart pointers are data structures that act like a pointer but also have additional metadata and capabilities
//in many cases smart pointers own the data they point to
//usually implemented using structs
//The Deref trait allows an instance of the smart pointer struct to behave like a reference so that you can write your code to work with either references or smart pointers. The Drop trait allows you to customize the code that’s run when an instance of the smart pointer goes out of scope. 
/*Box<T>, for allocating values on the heap
Rc<T>, a reference counting type that enables multiple ownership
Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time
*/


//using box<T> to point a data on the heap
/*boxes allows to store data on the heap 
on the stack is the pointer to the heap data
need of box:
when the size of a type cannot be known at compile time
When transferring ownership of large data
Allows storing different types implementing the same trait.*/
/*
fn main() {
    let b = Box::new(5);//adds unnecessary heap allocation.
    println!("b = {}", b);
}*/

//recursive type contain itself example==binary tree ,linked list 
/*
Breaking the Infinite Chain with Box
Instead of storing another List, store a pointer:
enum List {
    Cons(i32, Box<List>),
    Nil,
}*/
/*
use List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list =
        Cons(
            1,
            Box::new(
                Cons(
                    2,
                    Box::new(
                        Cons(
                            3,
                            Box::new(Nil)
                        )
                    )
                )
            )
        );
}*/
/*
#[derive(Debug)]
enum Tree {
    Node(i32, Box<Tree>, Box<Tree>),
    Empty,
}

use Tree::*;

fn main() {

    let tree =
        Node(
            10,
            Box::new(
                Node(
                    5,
                    Box::new(Empty),
                    Box::new(Empty),
                )
            ),
            Box::new(
                Node(
                    20,
                    Box::new(Empty),
                    Box::new(Empty),
                )
            ),
        );
        println!("{:?}", tree);

    let a = Box::new(10);

    let b = a;

    // println!("{}", a); // ERROR

    println!("{}", b);
}*/
#[derive(Debug)]
enum Tree {
    Node(i32, Box<Tree>, Box<Tree>),
    Empty,
}

use Tree::*;

fn main() {

    let tree =
        Node(
            10,
            Box::new(
                Node(
                    5,
                    Box::new(Empty),
                    Box::new(Empty),
                )
            ),
            Box::new(
                Node(
                    20,
                    Box::new(Empty),
                    Box::new(Empty),
                )
            ),
        );

    match tree {
        Node(value, left, right) => {
            println!("Root = {}", value);
            println!("Left subtree = {:?}", left);
            println!("Right subtree = {:?}", right);
        }

        Empty => println!("Empty"),
    }
}