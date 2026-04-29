//A slice is a kind of reference ,so it does not have ownership.
fn first_word_size(s: &str) -> &str {
    let bytes=s.as_bytes();//.as_bytes() converts the string into a byte slice
    for (i,&item) in bytes.iter().enumerate(){
        if item==b' '{
            return &s[0..i];
            //return i;
        }
    }
    &s[..]
   // s.len()
}
//iter is a method that returns each element in a collection and that enumerate wraps the result of iter and returns each element as part of a tuple 
//b is byte literal represnt a singe ascii byte . u8 type .(b'a'==97)
fn main(){
    let mut s=String::from("Helloo world!");
    let word = first_word_size(&s);
     println!("First word: {}", word);
    s.clear(); //this empties the string ,make it to equal to "";
}
//A string slice is a refernce to a contiguous sequence of the elements of a string 
//slice data structure stores the starting and the length of the string
