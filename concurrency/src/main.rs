//==========Creating a thread======
/*
use std::thread;
use std::time::Duration;
fn main(){
    thread::spawn(||{
        for i in 1..10 {
            println!("hi number{i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {i} is from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}*/
//the output will be ki main thread jb tk chlega tbhi tk spawn thread chlega
//A JoinHandle<T> is an owned value that, when we call the join method on it, will wait for its thread to finish. 
/*
use std::thread;
use std::time::Duration;
fn main(){
    let handle =thread::spawn(||{
        for i in 1..10 {
            println!("this number {i} is from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });
   // handle.join().unwrap();  // the main thread will wait for the spawned thread to complete
    for i in 1..5 {
        println!("this number {i} is from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
   handle.join().unwrap();
}*/
//using move closures with thread
/*
use std::thread;
fn main(){
    let v = vec![1,2,3];
    let handle = thread::spawn(||{
        println!("here is a vector: {v:?}");
    });
    handle.join().unwrap();
}
*/
/*
use std::thread;
fn main(){
    let v = vec![1,2,3];
    let handle = thread::spawn(move||{//it will tale ownership of the value
        println!("here is a vector: {v:?}");
    });
    handle.join().unwrap();
}*/

//Transfer data btw threads with message passing

//Creating Multiple Producers
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Got: {received}");
    }

}
