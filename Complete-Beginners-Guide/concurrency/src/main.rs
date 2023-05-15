use std::{thread, time::Duration, thread::sleep, sync::mpsc, sync::{Mutex, Arc}};


const NUM_THREADS: usize = 20;

fn main() {
    //threads
    let mut threads = vec![];
    for i in 0..10 {
        let th = thread::spawn(move || {
            sleep(Duration::from_millis(i * 100));
            println!("new thread {}", i);
        });
        threads.push(th);
    }

    for th in threads {
        th.join().unwrap();
    }

    println!("main thread");

    //channels
    // let (tx, rx) = mpsc::channel();
    // thread::spawn(move || {
    //     tx.send(42).unwrap();
    // });
    // println!("received {}", rx.recv().unwrap());

    let (tx, rx) = mpsc::channel();

    for i in 0..NUM_THREADS {
        start_thread(i, tx.clone());
    }

    for j in rx.iter().take(NUM_THREADS) {
        println!("received {}", j);
    }

    //mutexes
    let c = Arc::new(Mutex::new(0));
    let mut threads = vec![];

    for _i in 0..10 {
        let c = Arc::clone(&c);
        let t = thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num += 1;
            println!("{}", *num);
        });
        threads.push(t);
    }

    for th in threads {
        th.join().unwrap();
    }

    println!("Results {}", *c.lock().unwrap());
    
}

fn start_thread(d: usize, tx: mpsc::Sender<usize>) {
    thread::spawn(move || {
        println!("Setting timer {}", d);
        thread::sleep(Duration::from_millis(d as u64));
        println!("Sending {}", d);
        tx.send(d).unwrap();
    });
}
