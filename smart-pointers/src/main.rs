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

    match tree {
        Node(value, left, right) => {
            println!("Root = {}", value);
            println!("Left subtree = {:?}", left);
            println!("Right subtree = {:?}", right);
        }

        Empty => println!("Empty"),
    }
}*/
//=======Treating Smart Pointers Like Regular References=======
//derefernce Deref implementation
/*
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
 //   assert_eq!(5, *y); error bcs coparison between integer and address
} */
 //We can rewrite the code in Listing 15-6 to use a Box<T> instead of a reference; the dereference operator used on the Box<T>
/*
 fn main(){
    let x=3;
    let y=Box::new(x);
    assert_eq!(3,x);
    assert_eq!(3,*y);
}*/
//In Rust, assert_eq! is a macro used to check whether two values are equal. If they are not equal, the program panics and displays both values.

//definig own smart pointer
/*
struct MyBox<T>(T);//struct name is mybox and declare a generic parameter T 
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
    fn main() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
  //  assert_eq!(5, *y); //error cant be dereferenced

}*/


//implementing the deref trait
/*
use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
*/
//deref coercion converts a refernce to a type that implements the deref trait into a refernce to another type
/*
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
fn hello(name: &str) {
    println!("Hello, {name}!");
}
/*
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}*/
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}*/

//Hndling the deref coercion
/* 
Rust does deref coercion when it finds types and trait implementations in three cases:
From &T to &U when T: Deref<Target=U>
From &mut T to &mut U when T: DerefMut<Target=U>
From &mut T to &U when T: Deref<Target=U>
*/
/*
Deref coercion automatically converts references from one type to another compatible type using the Deref trait, making function and method calls easier.
Rust repeatedly calls deref() as many times as needed at compile time, so there is no runtime overhead.
For example, &MyBox<String> → &String → &str, allowing hello(&m) to work when hello expects a &str.
Mutable deref coercion uses the DerefMut trait and supports:
&T → &U
&mut T → &mut U
&mut T → &U
Rust allows converting mutable references to immutable references, but never the reverse (&T → &mut U), because doing so could violate Rust's borrowing rules and compromise memory safety.
*/


// The drop trait ============
/*
use std::mem::drop;//used for froce drop

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
/*
fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    // c.drop(); //error destructor
    println!("CustomSmartPointers created");
}*/
    /*
At the end of main, our instances of CustomSmartPointer will go out of scope, and Rust will call the code we put in the drop method, printing our final message. Note that we didn’t need to call the drop method explicitly.
*/
/* output :
CustomSmartPointers created
Dropping CustomSmartPointer with data `other stuff`!
Dropping CustomSmartPointer with data `my stuff`!
*/
//Variables are dropped in the reverse order of their creation

fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created");
    drop(c);//forced drop
    println!("CustomSmartPointer dropped before the end of main");
}*/
/*output:
CustomSmartPointer created
Dropping CustomSmartPointer with data `some data`!
CustomSmartPointer dropped before the end of main*/

//=====Refernce counted smart pointer==========
/*
Rc<T> allows multiple owners of the same data in a single-threaded program.
Box<T> → one owner
Rc<T> → many owners
Data is deleted automatically when the reference count becomes 0.*/
/*//With Box<T>, ownership is exclusive.
enum List {
    Cons(i32, Box<List>),
    Nil,
}
use List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(
                Cons(10, Box::new(Nil))));

    let b = Cons(3, Box::new(a)); // a moved here
    let c = Cons(4, Box::new(a)); // ERROR!
}*/
/*
use std::rc::Rc;
use List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(
        Cons(5,
            Rc::new(
                Cons(10,
                    Rc::new(Nil)
                )
            )
        )
    );

    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}*/
/*//all three share ownership
       Rc<List>
          a
      5 -> 10 -> Nil
       ^         ^
       |         |
   b:3           c:4    */

 //  Rc::clone() does not copy the data it only increase refernce count
/*
 use std::rc::Rc;
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
use List::{Cons, Nil};
fn main() {
    let a = Rc::new(
        Cons(5,
            Rc::new(
                Cons(10,
                    Rc::new(Nil)
                )
            )
        )
    );
    println!("count after creating a = {}",
             Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}",
             Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}",
                 Rc::strong_count(&a));
    } // c dropped here
    println!("count after c goes out of scope = {}",
             Rc::strong_count(&a));
}
// Limitation:Rc<T> only provides shared immutable access.*/


//========interior mutability======
//With RefCell<T>, the outside can be immutable, but the inside can still be modified.
/*
use std::cell::RefCell;
fn main() {
    let x = RefCell::new(5);
    *x.borrow_mut() += 1;
    println!("{}", x.borrow());
}
*/
//Borrow rules are checked at compile time
/*let mut x = 5;
let r1 = &mut x;
let r2 = &mut x; // Compiler error
*/
//RefCell<T>:Borrow rules are checked at runtime.
/*
use std::cell::RefCell;
fn main() {
    let x = RefCell::new(5);

    let a = x.borrow_mut();
    let b = x.borrow_mut(); // Compiles

}*/
//borrow() for refcell to work
//borrow_mut()
/*
let x = RefCell::new(5);
let m = x.borrow_mut();
*/ //Only one mutable borrow at a time.

//refcell keeps a counter. it can have many readrs but only one writer

//Rc<T> + RefCell<T>  =>which means:Multiple owners + Mutable shared data

use std::cell::RefCell;
use std::rc::Rc;
fn main() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::clone(&value);
    let b = Rc::clone(&value);
    *value.borrow_mut() += 10;
    println!("{}", a.borrow());
    println!("{}", b.borrow());
}//All owners observe the same change.
/*
Type	          Owners	    Mutable?	    Borrow Check
Box<T>	           One	           Yes	          Compile time
Rc<T>             Many	            No	          Compile time
RefCell<T>	       One	            Yes	            Runtime
Rc<RefCell<T>>	   Many	            Yes          	Runtime
Arc<Mutex<T>>	   Many threads	    Yes          	Runtime
*/