//======Genric data type======
/*
fn largest_i32(list: &[i32]) -> &i32 {//the function takes a slice of i32 values and returns a reference to the largest i32 value in the slice
    let mut largest = &list[0];//we initialize largest to be a reference to the first element of the list

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest//we return largest which is a reference to the largest i32 value in the slice
}

fn largest_char(list: &[char]) -> &char {//the function takes a slice of char values and returns a reference to the largest char value in the slice
    let mut largest = &list[0];//we initialize largest to be a reference to the first element of the list
//using unicode values to compare char values. The char with the largest unicode value will be considered the largest char.
    
for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest//we return largest which is a reference to the largest char value in the slice
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");
}*/

//now by using genric data type we can write a single function that can work with any data type that can be compared using the > operator. We can use the PartialOrd trait to specify that the type T must implement the > operator. This way we can find the largest value in a list of any type that implements the PartialOrd trait.
//std::cmp::PartialOrd//PartialOrd is a trait that allows us to compare values of a type using the > operator. By specifying that T must implement PartialOrd, we can use the > operator to compare values of type T in our largest function.
/*
fn largest<T: PartialOrd>(list: &[T]) -> &T {//the function takes a slice of values of type T and returns a reference to the largest value in the slice. The type T must implement the PartialOrd trait, which allows us to compare values of type T using the > operator.
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");
}*/

//======In Structs using genric data type======
/*
struct Point<T> {//T is a genric type parameter that can be used to specify the type of the fields in the Point struct. By using a genric type parameter, we can create a Point struct that can work with different types for its x and y fields, making it more flexible and reusable for different types of data.
    x: T,//T is a placeholder for a type that will be specified when we create an instance of the Point struct. This allows us to create Point instances with different types for x and y, such as Point<i32> for integer coordinates or Point<f64> for floating-point coordinates.
    y: T,// By using a genric type parameter T, we can create a Point struct that can work with any type for its x and y fields, as long as both fields are of the same type. This makes our Point struct more flexible and reusable for different types of data.
}

fn main() {
    let _integer = Point { x: 5, y: 10 };//Here we create an instance of the Point struct with x and y as integers. The type of the Point instance is inferred to be Point<i32> based on the values we provided for x and y.
    let _float = Point { x: 1.0, y: 4.0 };//    Here we create another instance of the Point struct with x and y as floating-point numbers. The type of this Point instance is inferred to be Point<f64> based on the values we provided for x and y.
}*/
// _ is used to indicate that the variable is intentionally unused. This can be useful to avoid compiler warnings about unused variables, while still allowing us to create instances of the Point struct with different types for x and y.
// However, if we try to create an instance of the Point struct with different types for x and y, such as Point { x: 5, y: 4.0 }, we will get a compile-time error because the type parameter T must be the same for both fields. This is because the genric type parameter T is defined for the entire struct, and all fields that use T must be of the same type. In this case, we would need to define a separate struct with two genric type parameters, such as Point<T, U>, to allow for different types for x and y.
/*
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 5, y: 4.0 };// This will not compile because the type parameter T must be the same for both fields x and y. In this case, we are trying to create a Point instance with x as an integer (5) and y as a floating-point number (4.0), which violates the requirement that both fields must be of the same type. To fix this, we would need to define a separate struct with two genric type parameters, such as Point<T, U>, to allow for different types for x and y.
}
*/
/*
struct Point<T, U> {// here we define a Point struct with two genric type parameters, T and U. This allows us to specify different types for the x and y fields, making our Point struct more flexible and reusable for different types of data.
    x: T,
    y: U,
}

fn main() {
    let _both_integer = Point { x: 5, y: 10 };// Here we create an instance of the Point struct with x and y as integers. The type of this Point instance is inferred to be Point<i32, i32> based on the values we provided for x and y.
    let _both_float = Point { x: 1.5, y: 4.9 };//    Here we create another instance of the Point struct with x and y as floating-point numbers. The type of this Point instance is inferred to be Point<f64, f64> based on the values we provided for x and y.
    let _integer_and_float = Point { x: 5, y: 4.8 };//   Here we create an instance of the Point struct with x as an integer and y as a floating-point number. The type of this Point instance is inferred to be Point<i32, f64> based on the values we provided for x and y.
    println!("Integer Point: ({}, {})", _both_integer.x, _both_integer.y);
    println!("Float Point: ({}, {})", _both_float.x, _both_float.y);
    println!("Integer and Float Point: ({}, {})", _integer_and_float.x, _integer_and_float.y);
}*/


//======In Enums using genric data type======
//option<t> is an enum that can be used to represent a value that can either be present (Some) or absent (None). By using a genric type parameter T, we can create an Option enum that can work with any type of value, making it more flexible and reusable for different types of data.
//result<T, E> is an enum that can be used to represent a value that can either be a success (Ok) or an error (Err). By using genric type parameters T and E, we can create a Result enum that can work with any type of success value and any type of error value, making it more flexible and reusable for different types of operations that may succeed or fail.


//======in methods using genric data type======
/*
struct Point<T> {// we define a Point struct with a genric type parameter T, which allows us to specify the type of the x and y fields when we create an instance of the Point struct. This makes our Point struct more flexible and reusable for different types of data.
    x: T,
    y: T,
}

impl<T> Point<T> {//impl<T> means that we are implementing methods for the Point struct that has a genric type parameter T. This allows us to define methods that can work with any type of Point, regardless of the specific type used for x and y.
    fn x(&self) -> &T {// we define a method named x that takes a reference to self (the instance of the Point struct) and returns a reference to the x field of the Point. The return type is &T, which means that the method will return a reference to a value of type T, which is the same type as the x field. By using a genric type parameter T, we can create a method that can work with any type of Point, making it more flexible and reusable for different types of data.
        &self.x//
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}*/
/*
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {// we define a method named mixup that takes ownership of self (the instance of the Point struct) and takes another Point instance as a parameter. The method returns a new Point instance with the x field from self and the y field from the other Point instance. By using genric type parameters X1, Y1, X2, and Y2, we can create a method that can work with any combination of types for the x and y fields of both Point instances, making it more flexible and reusable for different types of data.
        Point {
            x: self.x,// we use self.x to access the x field of the current Point instance and assign it to the x field of the new Point instance that we are creating. This allows us to mix the x value from self with the y value from the other Point instance. 
            y: other.y, // we use other.y to access the y field of the other Point instance and assign it to the y field of the new Point instance that we are creating. This allows us to mix the x value from self with the y value from the other Point instance.
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}*/