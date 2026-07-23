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
/*
use trpl::{Either, Html};
fn main() {
    let args: Vec<String> = std::env::args().collect();
    trpl::block_on(async {//trpl::block_on is a function that takes an asynchronous block of code and runs it to completion. It blocks the current thread until the asynchronous code has finished executing, allowing us to use async/await syntax in a synchronous context.
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
*/


//=========applying concurrency with async ==============

//creating  a new task with spawn_task
//use std::time::Duration;

//fn main(){
  //  trpl::block_on(async{
        /*async {
        trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        }
        ) ;
        for i in 1..5{
            
            println!("hi number {i} from the main task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }   
     }
        */
        /*
    let handle =trpl::spawn_task(async {
        for i in 1..10 {
            println!("hi number {i} from the first task!");
            trpl::sleep(Duration::from_millis(500)).await;  
        }
    });
    for i in 1..5{
        println!("hi number {i} from the main task!");
        trpl::sleep(Duration::from_millis(500)).await;
    }
    handle.await.unwrap();
*//*
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;
        */


            //=======Sending data btw two task using message passing=====
            /*
            let (tx,mut rx) = trpl::channel();
            let val = String::from("hi");
            tx.send(val).unwrap();
            let received = rx.recv().await.unwrap();
            println!("Got: {received}");
            */
            
           // let (tx, mut rx) = trpl::channel();
            /*
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("other"),
                String::from("side"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
            while let Some(value) = rx.recv().await {
                println!("Got: {value}");
            }
*/
/*
        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
*/
//======= joining a number of futures with the join! macro

       // let (tx, mut rx) = trpl::channel();
/*
        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        trpl::join!(tx1_fut, tx_fut, rx_fut);

}
);

}*/

//=========yielding control to the runtime =========
//An async function does not automatically switch to another task.
//It keeps running until it reaches an .await.


//=====  starvation problem=====
/*
let a = async {
    slow("A", 30);
    slow("A", 20);
    slow("A", 10);
};
let b = async {
    println!("B started");
};
trpl::select(a, b).await;*/


// solution is to add await
/*
let one_ms = Duration::from_millis(1);
let a = async {
    slow("A",30);
    trpl::sleep(one_ms).await;
    slow("A",20);
    trpl::sleep(one_ms).await;
    slow("A",10);
};*/
// its called cooperative multitasking


// we can also use the yield_now function to yield control to the runtime, allowing other tasks to run. This is useful when we have a long-running task that does not have any .await points, and we want to give other tasks a chance to run.
//cooperative multitasking is each future decides when to pause itself
//Unlike threads, the OS doesn't force switching—the future cooperates by reaching an await.

//timeout is a way to limit how long a future can run before it is cancelled. This is useful when we have a future that may take a long time to complete, and we want to avoid blocking other tasks indefinitely. We can use the timeout function to wrap a future and specify a maximum duration for it to run. If the future does not complete within the specified duration, it will be cancelled and return an error.
//The timeout function is implemented using a combination of the select and sleep functions. It creates a new future that waits for either the original future to complete or the specified duration to elapse. If the original future completes first, it returns its result. If the duration elapses first, it returns an error indicating that the future timed out.
//Runs multiple futures at the same time, but returns the one that finishes first.

//Rust switches tasks only at .await points. Without an await, the current task keeps running and other tasks cannot make progress.

//sleep() pauses execution for a duration.
//yield_now() immediately hands control back to the runtime without waiting, making it more efficient when you just want other tasks to run.






