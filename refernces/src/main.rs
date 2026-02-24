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