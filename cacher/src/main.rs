use std::thread;
use std::time::Duration;

mod caching;

fn main() {
    let mut c: caching::Cacher<u32, u32> = caching::Cacher::new(Box::new(|a| {
        println!("Processing in closure....");
        thread::sleep(Duration::from_secs(1));
        println!("Done!");
        *a
    }));

    for _i in 0..2 {
        for j in 0..4 {
            println!("Value for {} is {}", j, *c.value(j));
        }
        println!("NO MORE SLEEPING");
    }
}
