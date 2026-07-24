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
}