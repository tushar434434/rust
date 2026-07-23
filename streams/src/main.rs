// a stream is a sequence of values that become available asynchronously
//Unlike an iterator, a stream may need to wait before the next value is available.
/*Iterator    vs            Stream
Synchronous	              Asynchronous
Uses next()	              Uses next().await
Data already available	  Data arrives over time
No waiting	              May wait for the next item
*/
/*StreamExt is an extension trait.
It adds useful methods to the basic Stream trait, including:
next()
map()
filter()
take()*/
/*Streams are useful when data arrives gradually, such as:
Chat messages
API responses
File chunks
Sensor data
User events*/

//Instead of waiting for all data, you can process each item as it arrives.
/*use trpl::StreamExt;
fn main() {
    trpl::block_on(async {
        let numbers = [1, 2, 3, 4, 5];
        let iter = numbers.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);
        while let Some(value) = stream.next().await {
            println!("{value}");
        }
    });
}*/
/*
use trpl::StreamExt;
fn main() {
    trpl::block_on(async {
        let words = vec!["Rust", "is", "awesome"];
        let iter = words.into_iter();
        let mut stream = trpl::stream_from_iter(iter);
        while let Some(word) = stream.next().await {
            println!("{word}");
        }
    });
}
    */
//=======using filter()=======
/*
use trpl::StreamExt;
fn main() {
    trpl::block_on(async {
        let numbers = 1..=10;
        let iter = numbers.filter(|x| x % 2 == 0);
        let mut stream = trpl::stream_from_iter(iter);
        while let Some(num) = stream.next().await {
            println!("{num}");
        }
    });
}*/
//use trpl::StreamExt;
fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();
        tx.send(String::from("Hello")).unwrap();
        tx.send(String::from("Rust")).unwrap();
        drop(tx); // Close the channel
        while let Some(msg) = rx.recv().await {
            println!("{msg}");
        }
    });
}