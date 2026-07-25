//an object contains data (feilds) and methods (functions) that operate on that data\
//in rust encapsulation is supported and encapsulation means it hide internal implementation and expose only necessary methods.
//rust use pub and privates keywords to control the visibility of struct fields and methods. By default, all fields and methods are private, meaning they can only be accessed within the same module. To make a field or method public, you can use the pub keyword before its declaration. This allows other modules to access it.

//inhertance is not supported in rust but it can be achieved using traits. A trait is a collection of methods that define a common behavior for types. A struct can implement a trait to inherit its methods and provide its own implementation. This allows for code reuse and polymorphism, as different structs can implement the same trait in different ways.
//polymorphism is supported in rust through traits. A trait defines a set of methods that a type must implement, and different types can implement the same trait in different ways. This allows for code reuse and flexibility, as functions can accept parameters of any type that implements a specific trait, enabling polymorphic behavior.
/*trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn print_area(shape: &impl Shape) {
    println!("{}", shape.area());
}

fn main() {
    let c = Circle { radius: 5.0 };
    let r = Rectangle {
        width: 4.0,
        height: 6.0,
    };

    print_area(&c);
    print_area(&r);
}
    */
//Polymorphism means:One interface, many implementations.


//=====Traits========
/*
trait Draw {
    fn draw(&self);
}

struct Button {
    label: String,
}

struct TextField {
    placeholder: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button: {}", self.label);
    }
}

impl Draw for TextField {
    fn draw(&self) {
        println!("TextField: {}", self.placeholder);
    }
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for c in &self.components {
            c.draw();
        }
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                label: "OK".into(),
            }),
            Box::new(TextField {
                placeholder: "Enter Name".into(),
            }),
        ],
    };
    screen.run();
}*/

//Box?  =>Trait objects don't have a known size at compile time. So Rust stores them behind a pointer.
//dynamic dispatch =>When a trait object is used, Rust uses dynamic dispatch to determine which implementation of a method to call at runtime. This allows for polymorphism, as different types can implement the same trait in different ways, and the correct implementation is chosen based on the actual type of the object at runtime.
/*Trait objects allow Rust to achieve runtime polymorphism without inheritance.
A trait defines shared behavior, such as Draw with a draw() method.
Different types (e.g., Button, TextField) can implement the same trait.
Box<dyn Draw> can store any type that implements the Draw trait.
This allows a single Vec to hold different component types together.
The Screen struct calls draw() on each component without knowing its concrete type.
Trait objects use dynamic dispatch, so the correct method is chosen at runtime.
Generics use static dispatch, which is faster but only supports one concrete type per collection.
Rust ensures at compile time that only types implementing the required trait can be used.
Trait objects provide flexibility for heterogeneous collections, while generics provide better performance for homogeneous collections.*/
//

//The State Pattern is an Object-Oriented Design Pattern where an object's behavior changes depending on its current state.
/*Rust implements it using traits and trait objects instead of inheritance.
A blog post has three states: Draft, PendingReview, and Published.
A new post always starts as a Draft.
request_review() changes the state to PendingReview.
approve() changes the state to Published.
Only Published posts can display their content.
The traditional implementation stores the state using Box<dyn State>.
Rust's preferred approach uses different types (DraftPost, PendingReviewPost, Post) to represent states.
Encoding states in the type system prevents invalid operations at compile time, making Rust programs safer.*/

/*
trait State {
    fn publish(&self) -> &'static str;
}

struct Draft;
struct PendingReview;
struct Published;

impl State for Draft {
    fn publish(&self) -> &'static str {
        "Post is still in Draft."
    }
}

impl State for PendingReview {
    fn publish(&self) -> &'static str {
        "Post is under Review."
    }
}

impl State for Published {
    fn publish(&self) -> &'static str {
        "Post is Published!"
    }
}

fn main() {
    let draft = Draft;
    let review = PendingReview;
    let published = Published;

    println!("{}", draft.publish());
    println!("{}", review.publish());
    println!("{}", published.publish());
}*/

//   Blog workflow
struct DraftPost {
    content: String,
}

struct PendingReviewPost {
    content: String,
}

struct Post {
    content: String,
}

impl DraftPost {
    fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

impl Post {
    fn content(&self) -> &str {
        &self.content
    }
}

fn main() {
    let mut draft = DraftPost::new();

    draft.add_text("Learning Rust State Pattern");

    let review = draft.request_review();

    let post = review.approve();

    println!("Published Post: {}", post.content());
}