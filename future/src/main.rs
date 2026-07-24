//evry async fn or async block returns a future, which is a value that represents a computation that may not have completed yet. A future is a type that implements the Future trait, which has a poll method that can be used to check if the computation is complete and to retrieve the result when it is ready. Futures are lazy, meaning they do not start executing until they are awaited or polled.
/*pub trait Future {
    type Output;
    fn poll(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output>;
}*/
//Output → Final value returned.
//poll() → Checks whether the future is finished.
//poll() always returns 
/*
use tokio::time::{sleep, Duration};
#[tokio::main]
async fn main() {
    println!("Task started");
    sleep(Duration::from_secs(2)).await;
    println!("Task finished");
}*/

// never call poll() yourself.
// The runtime does it automatically.

//pin!() means ::> Don't move this value in memory.
//Async futures can be self-referential. Moving them could invalidate internal references, so Pin guarantees they stay at a fixed memory location.
//Unpin is a marker trait indicating that a type is safe to move, even when behind a Pin. Most standard Rust types implement it automatically.
//Use threads for CPU-bound work, such as video encoding, image processing, compression, and scientific calculations.
//Use async for I/O-bound work, such as HTTP requests, file I/O, database queries, sockets, and chat applications.
//A Future is the smallest unit of async work. Futures are grouped into tasks, and tasks are scheduled by the runtime.
