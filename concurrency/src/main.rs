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

use std::thread;
use std::time::Duration;
fn main(){
    let handle =thread::spawn(||{
        for i in 1..10 {
            println!("this number {i} is from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("this number {i} is from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}