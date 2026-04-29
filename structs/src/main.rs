//struct is the key word and all will be in curly brackets
//structs contains key value pairs
/*
struct User {
active: bool,
name:String,
email:String,
Sign_in_count:u64,
}
fn main(){
    let User1=User{
        active:true,
        name:String::from("Tushar"),
        email:String::from("tushar@gmail.com"),
        Sign_in_count:1,
    };
    let user2 = User {
        email: String::from("another@example.com"),
        ..User1
    };
}*/
// an eg program using structs
/*
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}*/
//refactoring the above program using tuples
/*
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}*/
//refactoring the above program using structs
/*
struct Rectangle {//add meaning by labeling the data.
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2=Rectangle{
        width:20,
        height:40,
    };
   // println!("rect1 is {rect1}");//error because the struct does not have the trait of printing and we can not print it directly but we can print its fields like rect1.width and rect1.height
   // println!("rect2 is {rect2}");

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect2)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}*/
/*
#[derive(Debug)]//this is a trait that allows us to print the struct using {:?} in println! macro and {:#?} for pretty printing in next line
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}");
}*/
//anohter way using dbg! macro to print the struct
/*
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);//this will print the value of rect1 and also the file and line number where it is called
}*/



//-----------METHODS-----------------
//they are similar to functions but they are defined within the context of a struct and they have access to the data of the struct and they are called using the dot notation on an instance of the struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
/*mpl block → define methods for Rectangle
Method vs Function → method is inside impl, function is outside
self → represents instance of struct
&self → immutable borrow (read only)
&mut self → mutable borrow (can modify)
self → takes ownership (rare use)
Self** → alias for struct type (Rectangle`)
Method syntax → rect1.area() (dot operator)
No need to pass object → unlike function (area(rect1))
Encapsulation → related functions grouped in impl
Cleaner code organization
Borrowing concept → avoids ownership transfer*/