//=== Traits ===
//Traits define shared behavior for types in Rust.
//Trait Definition
pub trait Summary{
    fn summarize(&self)->String;//method signature only

    //default implementation
    fn summarize_author(&self)->String{
        String::from("(author unknown)")
    }
}
/*trait → defines behavior
traits are similar to interfaces
only method signature is written
implementing type provides body
default implementation can also be provided*/

//=== Type Definitions ===

pub struct NewsArticle{
    pub headline:String,
    pub location:String,
    pub author:String,
    pub content:String,
}

pub struct SocialPost{
    pub username:String,
    pub content:String,
}

//=== Trait Implementations ===

impl Summary for NewsArticle{

    fn summarize(&self)->String{
        format!("{}, by {}",self.headline,self.author)
        //format!() returns String
        //no semicolon because last expression is returned
    }

    fn summarize_author(&self)->String{
        format!("@{}",self.author)//@
    }
}

impl Summary for SocialPost{

    fn summarize(&self)->String{
        format!("{}: {}",self.username,self.content)
    }

    fn summarize_author(&self)->String{
        format!("@{}",self.username)
    }
}

//=== Using Traits ===

fn main(){

    let post=SocialPost{
        username:String::from("tushar"),
        content:String::from("hello"),
    };

    println!("{}",post.summarize());

    let article=NewsArticle{
        headline:String::from("Penguins win the Stanley Cup Championship!"),
        location:String::from("Pittsburgh, PA, USA"),
        author:String::from("Iceburgh"),
        content:String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };

    println!("{}",article.summarize());

    println!("{}",article.summarize_author());
}

/*Important Notes
impl Trait for Type
one trait can be implemented for many types
trait methods work like normal methods
format!() creates and returns String
last expression without semicolon is returned automatically
Default Implementation:
trait methods can already contain body
implementing type may override it
Orphan Rule:
allowed:
impl Display for MyType
not allowed:
impl Display for Vec<T>
because both trait and type are external.*/

//usimg traits as parameters
/*pub fn notify(item: &impl Summary) {//trait bound syntax
    println!("Breaking news! {}", item.summarize());
}*/

//trait bound syntax can also be written using where clause
//multiple trait bounds with + syntax
//pub fn notify(item: &(impl Summary + Display)) {
//pub fn notify(item: &(impl Summary + Display)) {

//=======clearer with where clause=======
//fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
/*fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
*/

//using trait bounds to conditionally implement methods

//blanket implementations are implementations that apply to any type that implements a certain trait

