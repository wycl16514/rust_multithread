In this section let's explore the multi-thread capability of rust and it is the shinning point of rust.Rust claims that you can have fearless 
multi-threading and have a sweet dream at night when you multi-thread application is running. Let's see how rust achives this by writting a piece
of code and dive deep into it:
```r
use std;
use std::thread;
fn run_thread() {
    let s = "string in run_thread";
    let mut v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        /*
        the key word move transfer the ownership of s from
        run_thread into this closure, the sleep will cause
        the thread suspend and when it wakes up, run_thread
        alread complete and s is created by this function,
        then it will destrop when this function complete.
        if we don't move the ownership s to the closure,
        when the thread is running it will reference to an invalid
        object, the move cause s not to be destroyed when the function
        is complete
         */
        thread::sleep(std::time::Duration::from_millis(2000));
        println!("the content of s : {}", s);
        v.push(4);
        println!("the content of v: {:?}", v);
        //uncomment the following code and check the effect of unwrap and expect
        //println!("error here: {}", v[6]);
    });

    //change v here is illegal, you can't modify the thing you don't own
    //this is the power for fearless multi-threading
    //v.push(4);

    /*
    join ask the main thread wait until the given thread above complete.
    unwrap causes the program crash immediately without given any error
    message
     */
    handle.join().unwrap();
    //when the thread has error that causes the program crash, the string
    //in expect will print out.
    //handle.join().expect("the thread created in run_thread panic");
}

fn main() {
    run_thread();
}

```
we need to follow many rules when doing multi-threading in rust, first is that we need the keyword "move" to transfer the ownship of captured variable to the closure.

Because the lifetime of the thread may longer than the function, variables created in the function will destroy when the function is completed.

But at that time the thread may referencing those variables that will cause the referencing of invalid memory and cause the application to crash
or behavior maddly.The effectiveness of memory referenced by multi-thread is a Prime culprit for the crash of c++ multi-threading application.That's why designing multi-thread application by rust is much safter and easier than c++.

Now let's see the difference between unwrap and expect, basically these two guys are the same, when there are errors in the thread, they both
cause the application to crash but expect will output the string inside it when crashing the application. If we uncomment that line of code in
the closure and use unwrap at the end, you will see following:

<img width="1062" alt="截屏2024-03-10 22 59 09" src="https://github.com/wycl16514/rust_multithread/assets/7506958/03d0513d-353e-4321-b31a-5cf33c82fb00">

Following are shown by using expect:
<img width="1052" alt="截屏2024-03-10 23 02 13" src="https://github.com/wycl16514/rust_multithread/assets/7506958/9ad87b58-6323-48fc-a7ae-73c943c6c7fa">

Finally what's difference between async and threading? Not much, if you application is computation entensive, then use threading, if it is IO 
entensive, like requesting data from remote database, use async.

