//Asynchronous programming is an abstraction that lets us express our code in terms of potential pausing points and eventual results that takes care of the details of coordination for us.
//async :some opertaions take a long time.
//cpu-bound task: operations that utilize the CPU for extended periods.
//eg vedio rendering image processing, data analysis, etc.
//i-o-bound task: operations that spend most of their time waiting for external resources to respond.
//eg network requests, file system operations, database queries, etc.
//async mainly designed to improve the performance of i-o-bound tasks by allowing other tasks to run while waiting for external resources to respond.
//normal functions are blocking:
//we can use threads to achieve parallelism and concurrency, but it can be complex and error-prone.consume memory expensive and thousand of thrads become inefficient.
//bettr option is to use async programming, which allows us to write non-blocking code that can handle many tasks concurrently without the overhead of threads.
//parallelism and concurrency=========
/* Concurrency	                  Parallelism
Taskstake turns	                  Tasks run simultaneously
Possible on one CPU core	      Needs multiple CPU cores
Focuses on scheduling	          Focuses on execution*/
