use std::{sync::{mpsc, Arc, Mutex}, thread};
use crate::hash::hash_value;

pub fn worker_thread(counter: Arc<Mutex<usize>>, stop_flag: Arc<Mutex<bool>>, suffix: String, tx: mpsc::Sender<(usize, String)>) {
    loop {
        if *stop_flag.lock().unwrap() {
            break;
        }

        let num = {
            let mut counter = counter.lock().unwrap();
            *counter += 1;
            *counter
        };

        let hashed_value = hash_value(num);
        if hashed_value.ends_with(&suffix) {
            if let Err(_) = tx.send((num, hashed_value)) {
                break;
            }
        }
    }
}

pub fn start_threads(count_suffix: usize, tx: mpsc::Sender<(usize, String)>, stop_flag: Arc<Mutex<bool>>) -> Vec<thread::JoinHandle<()>> {
    let num_threads = num_cpus::get();
    let suffix = "0".repeat(count_suffix);
    let counter = Arc::new(Mutex::new(0));

    (0..num_threads)
        .map(|_| {
            let counter = Arc::clone(&counter);
            let stop_flag = Arc::clone(&stop_flag);
            let suffix = suffix.clone();
            let tx = tx.clone();

            thread::spawn(move || {
                worker_thread(counter, stop_flag, suffix, tx);
            })
        })
        .collect()
}