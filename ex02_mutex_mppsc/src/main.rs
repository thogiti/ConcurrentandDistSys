// Import the Mutex type from the parking_lot crate for efficient locking.
use parking_lot::Mutex;
// Import the channel and Arc from the standard library for multi-threading and sharing state.
use std::sync::{mpsc::channel, Arc};
use std::thread;

// Define a constant N representing the desired value to reach.
const N: usize = 10;

// Main entry point of the program.
fn main() {
    // Wrap an integer in a Mutex and Arc to allow safe, shared mutation across threads.
    let data = Arc::new(Mutex::new(0));
    // Create a channel for sending signals between threads.
    let (tx, rx) = channel();

    // Spawn ten threads.
    for _ in 0..N {
        // Clone the Arc to allow each thread to have its own reference to the Mutex.
        let (data, tx) = (Arc::clone(&data), tx.clone());

        // Spawn a new thread with move semantics to capture the cloned Arc and channel sender.
        thread::spawn(move || {
            // Lock the Mutex to gain mutable access to the inner data.
            let mut data = data.lock();
            // Increment the data.
            *data += 1;
            // If the data reaches the desired value N, send a unit value through the channel.
            if *data == N {
                tx.send(()).unwrap();
            }
        });
    }

    // Wait for a message from any of the spawned threads indicating that the data has reached N.
    rx.recv().unwrap();
    // After receiving the signal that data has reached N, lock the Mutex once more and print the data.
    println!("Final value of data: {}", *data.lock());
}
