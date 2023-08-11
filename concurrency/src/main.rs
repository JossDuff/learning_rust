use std::sync::mpsc; // channels
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // basic thread
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    // thread taking ownership of a value
    let v = vec![1, 2, 3];

    let handle1 = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle1.join().unwrap();

    // “Do not communicate by sharing memory; instead, share memory by communicating.”
    // -Go language docs
    // MESSAGE PASSING THREADS
    let (tx, rx) = mpsc::channel();
    // move bcs this thread has to own tx
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // send val from thread
    });

    // receive val from thread.  .recv() block's this thread (main) until message or error is received
    // try_recv() doesn't block (use it in a loop to periodically check for a message! :))
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // more ex of message passing
    let (tx1, rx1) = mpsc::channel();

    thread::spawn(move || {
        let vals1 = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val1 in vals1 {
            tx1.send(val1).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received1 in rx1 {
        println!("Got: {}", received1);
    }

    // multiple threads all sending messages
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
        println!("Got: {}", received);
    }

    // SHARED STATE THREADS
    // mutex
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap(); // acquire lock
        *num = 6;
    }

    println!("m = {:?}", m);

    // atomic reference counting with arc
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

    println!("Result: {}", *counter.lock().unwrap());

    // because very little of how Rust handles concurrency is part of the language,
    // many concurrency solutions are implemented as crates. These evolve more
    // quickly than the standard library, so be sure to search online for the
    // current, state-of-the-art crates to use in multithreaded situations
}
