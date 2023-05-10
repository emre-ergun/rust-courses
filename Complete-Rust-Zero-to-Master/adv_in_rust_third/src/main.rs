use std::thread;
use std::time::Duration;

fn main() {
	let park_thread = thread::Builder::new().spawn(
		||{
			println!("Parking Thread!");
			thread::park();
			println!("Thread unparked!");
		}).unwrap(); //give me the result and if there is error panic and stop the prg.

	thread::sleep(Duration::from_millis(100));
	
	println!("Unpark the Thread!");
	park_thread.thread().unpark();
    park_thread.join().unwrap(); 


    let x = thread::spawn(
        ||{
            for i in 1..20 {
                println!("Spawn thread: {}", i);
                thread::sleep(Duration::from_millis(1));
            }
        }
    );

    for i in 1..5 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(2));
    }

    x.join().unwrap();

}