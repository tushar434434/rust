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
use std::thread;
fn main(){
    let v = vec![1,2,3];
    let handle = thread::spawn(move||{//it will tale ownership of the value
        println!("here is a vector: {v:?}");
    });
    handle.join().unwrap();
}