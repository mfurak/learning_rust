use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let counter1 = Arc::new(Mutex::new(0));
    let counter2 = Arc::new(Mutex::new(0));

    let counter1_thread1 = Arc::clone(&counter1);
    let counter2_thread1 = Arc::clone(&counter2);
    let handle1 = thread::spawn(move || {
        let mut num = counter1_thread1.lock().unwrap();
        thread::sleep(Duration::from_secs(1));
        let mut num2 = counter2_thread1.lock().unwrap();
        *num += 1;
        *num2 += 1;
    });
    
    let counter1_thread2 = Arc::clone(&counter1);
    let counter2_thread2 = Arc::clone(&counter2);
    
    let handle2 = thread::spawn(move || {
        let mut num = counter2_thread2.lock().unwrap();
        thread::sleep(Duration::from_secs(1));
        let mut num2 = counter1_thread2.lock().unwrap();
        *num += 1;
        *num2 += 1;
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("counter1: {}, counter2: {}", *counter1.lock().unwrap(), *counter2.lock().unwrap());
}