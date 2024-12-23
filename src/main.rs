mod args;
mod hash;
mod worker;

use std::sync::{mpsc, Arc, Mutex};

fn main() {
    let (n, f) = args::parse_input();
    let (tx, rx) = mpsc::channel();
    let stop_flag = Arc::new(Mutex::new(false));

    let handles = worker::start_threads(n, tx, Arc::clone(&stop_flag));

    for (num, hashed_value) in rx.iter().take(f) {
        println!("Number: {}, Hash: {}", num, hashed_value);
    }

    *stop_flag.lock().unwrap() = true;

    for handle in handles {
        handle.join().unwrap();
    }

}