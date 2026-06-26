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
/*
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

}*/
//Shared state concurrency 
//multiple threads work on the same piece of data .
//Mutex (Mutual Exclusion) is a synchronization primitive that allows only one thread at a time to access shared data.
//mutex has two rules lock before accessing the data and the second is unlock after finishing rust this automatically when the lock goes out of scope.
/*
use std::sync::Mutex;
fn main(){
    let m =Mutex::new(5);
    {
        let mut value =m.lock().unwrap();
        *value =10;
    }
    println!("{:?}",m);
}*/
//Arc<T>:Arc stands for Atomic Reference Counted.
//It is the thread-safe version of Rc
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("{}", *counter.lock().unwrap());
}
/*
Send → Ownership can be safely moved between threads.
Sync → Shared references (&T) can be safely accessed by multiple threads.
Rc<T> is not Send or Sync because its reference counting is not thread-safe.
Arc<T> is thread-safe because it uses atomic reference counting.
Mutex<T> allows only one thread to access shared data at a time, making shared mutation safe.
Most primitive and standard Rust types automatically implement Send and Sync.
Manual implementation of these traits is unsafe and only needed for advanced concurrency primitives.*/