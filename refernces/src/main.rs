fn main() {
   /*I let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");//refernces also immutable 
    */
    let mut s =String::from("Hello!");
    change(&mut s);
    println!("{s}");
}
fn change(s_s:&mut String){
    s_s.push_str(",world");
}
// If you have a mutable reference to a value, you can have no other references to that value.
//it prevent data race at copmpile time; race conditions===>
// Two or more pointers access the same data at the same time.
//At least one of the pointers is being used to write to the data.
//There’s no mechanism being used to synchronize access to the data.
//  let mut s = String::from("hello");

//     let r1 = &mut s;
//     let r2 = &mut s; not allowed passing refernce of s again ;
//we can use curly brackets to create a new scope, allowing for multiple mutable references,

//     println!("{r1}, {r2}"); 
//we can pass multiple reference of immutable vlaue