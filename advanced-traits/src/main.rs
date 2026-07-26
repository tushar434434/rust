//An associated type is a placeholder type inside a trait. The type is specified when the trait is implemented.
//It avoids writing generic types repeatedly.
/*trait MyIterator {
    type Item;

    fn next(&self) -> Self::Item;
}
struct Counter;
impl MyIterator for Counter {
    type Item = u32;
    fn next(&self) -> Self::Item {
        100
    }
}
fn main() {
    let c = Counter;
    println!("{}", c.next());
}*/
//default genric parameters   
// default generic parameter provides a default type if no type is specified.
/*
use std::ops::Add;//operator overloadinggggg
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
fn main() {
    let p1 = Point { x: 2, y: 3 };
    let p2 = Point { x: 4, y: 5 };

    let p3 = p1 + p2;

    println!("{:?}", p3);
}*/

//Operator overloading means changing the behavior of operators like +, -, *, etc., by implementing traits from std::ops.
/*
use std::ops::Add;
struct Number(i32);
impl Add for Number {
    type Output = Number;
    fn add(self, other: Number) -> Number {
        Number(self.0 + other.0)
    }
}
fn main() {
    let a = Number(10);
    let b = Number(20);

    let c = a + b;

    println!("{}", c.0);
}*/
//Disambiguating methods ======
//If multiple traits (or a trait and a struct) have methods with the same name, Rust requires you to specify which one to call.
/*trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
struct Human;
impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot flying");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard flying");
    }
}
impl Human {
    fn fly(&self) {
        println!("Human flying");
    }
}
fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}*/

//fully qualified syntax
//Fully qualified syntax tells Rust exactly which trait implementation to use.
/*
trait Animal {
    fn baby_name() -> String;
}
struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("Puppy")
    }
}
fn main() {
    println!("{}", Dog::baby_name());
    println!("{}", <Dog as Animal>::baby_name());
}*/

//Supertraits=======
//A supertrait is a trait that depends on another trait.
//A type must implement the parent trait before it can implement the child trait.
/*
use std::fmt;
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        println!("************");
        println!("* {} *", self);
        println!("************");
    }
}
struct Point {
    x: i32,
    y: i32,
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl OutlinePrint for Point {}
fn main() {
    let p = Point { x: 1, y: 2 };
    p.outline_print();
}
*/
//Newtype pattern ============
/*The Newtype Pattern wraps an existing type inside a tuple struct.
It is mainly used to:
Implement external traits
Follow Rust's orphan rule
Improve type safety*/

use std::fmt;
struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.join(", "))
    }
}
fn main() {
    let names = Wrapper(vec![
        String::from("Rust"),
        String::from("Go"),
        String::from("Python"),
    ]);
    println!("{}", names);
}
//Associated types allow a trait to be implemented only once for a given type, avoiding ambiguity and reducing the need for type annotations.
//It is customizing operators like + or - by implementing traits from std::ops, such as Add.
//The Orphan Rule is a Rust rule that prevents you from implementing an external trait for an external type.
//You can implement a trait only if at least one of the trait or the type belongs to your crate.