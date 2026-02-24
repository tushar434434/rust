fn main() {
       {                      // s is not valid here, since it's not yet declared
        let  _s= "hello";   // s is valid from this point forward

        // do stuff with s
        /*There is a natural point at which we can return the memory our String needs to the allocator: when s goes out of scope. */
     
                  }         // this scope is now over, and s is no longer valid
                  let _v = String::from("hello");//create a String from a string literal using the from function
                  let mut m=String::from("hello");
                  m.push_str(",world!");// push_str() appends a literal to a String
                  println!("updated value:{m}");
    // let s1 = String::from("hello");
    // let s2 = s1;//s1 moved into s2
    //error because s1 has been dropped automaticly after s2=s1;
    // println!("{s1}, world!");
                
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // Because i32 implements the Copy trait,
                                    // x does NOT move into the function,
                                    // so it's okay to use x afterward.

} // Here, x goes out of scope, then s. However, because s's value was moved,
  // nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

//=========mutable references==============
fn mutable_refrence() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


//======== OWNERSHIP=========
//Ownership is a set of rules that govern how a Rust program manages memory.
// Memory is managed through a system of ownership with a set of rules that the compiler checks.
//THREE RULES OF OWNERSHIP:======
//each value in rust has a owner.
//there can only be one owner at a time
//when the owner goes out of scope the value will be dropped.
/*Feature	    &str (String Literal)	      String
Memory	         Read-only (binary)	          Heap
Mutable	            ❌	                     ✅ (if mut)
Growable            ❌	                     ✅
Ownership	      Borrowed	                 Owned
Use Case	       Fixed text	              Editable text*/