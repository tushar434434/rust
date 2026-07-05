//trpl is short for “The Rust Programming Language”). It re-exports all the types, traits, and functions you’ll need, primarily from the futures and tokio crates
//defining the page-title function
/*
use trpl::Html;
async fn page_title(url:&str)->Option<String>{
    let response =trpl::get(url).await;
    //let response_text =response.text().await;
    let response_text = trpl::get(url).await.text().await;
    Html::parse(&response_text)
    .select_first("title")
    .map(|title| title.inner_html())
}*//*
use std::future::Future;//future is a trait that represents a value that may not have been computed yet. It is the core abstraction of asynchronous programming in Rust. A future is a value that will be available at some point in the future, and it can be used to represent the result of an asynchronous computation.
use trpl::Html;
fn page_title(url: &str) -> impl Future<Output = Option<String>> {//impl Future<Output = Option<String>> means that the function returns a type that implements the Future trait, with an output type of Option<String>. This allows the function to be used in an asynchronous context, where it can be awaited to get the result of the computation.
    async move {//async move block is used to create an asynchronous block of code that can be executed concurrently with other tasks. The move keyword is used to indicate that the variables captured by the block should be moved into the block, rather than borrowed. This is necessary because the block may outlive the current scope, and we want to ensure that the variables are still valid when the block is executed.
        let text = trpl::get(url).await.text().await;//trpl::get(url).await returns a Future that resolves to a Response object, which represents the HTTP response from the server. The .text().await method is called on the Response object to get the response body as a String. This is also an asynchronous operation, so we use .await to wait for it to complete.
        Html::parse(&text)// Html::parse(&text) is a method that takes a string of HTML and returns a parsed representation of the HTML document. This allows us to query the document for specific elements, such as the title tag.
            .select_first("title")//select_first("title") is a method that takes a CSS selector and returns the first element in the document that matches the selector. In this case, we are looking for the first <title> element in the HTML document.
            .map(|title| title.inner_html())//map(|title| title.inner_html()) is a method that takes a closure and applies it to the value inside the Option returned by select_first. If select_first returns Some(title), then the closure is called with the title element, and we return the inner HTML of the title element as a String. If select_first returns None, then map does nothing and we return None.
    }
}*/
/*
async fn main() {//will give error because main function is not async by default. We need to use an async runtime like tokio or async-std to run the main function asynchronously. We can use the #[tokio::main] attribute to mark the main function as asynchronous and use the tokio runtime to execute it.
    let args: Vec<String> = std::env::args().collect();
    let url = &args[1];
    match page_title(url).await {
        Some(title) => println!("The title for {url} was {title}"),
        None => println!("{url} had no title"),
    }
}*/
/*
fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::block_on(async {//trpl::block_on is a function that takes an asynchronous block of code and runs it to completion. It blocks the current thread until the asynchronous code has finished executing, allowing us to use async/await syntax in a synchronous context.
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    })
}*/
//Racing Two URLs Against Each Other Concurrently
use trpl::{Either, Html};
fn main() {
    let args: Vec<String> = std::env::args().collect();
    trpl::block_on(async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);
        let (url, maybe_title) =
            match trpl::select(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };
        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title was: '{title}'"),
            None => println!("It had no title."),
        }
    })
}
async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}


