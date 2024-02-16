// Import the Condvar and Mutex types from the parking_lot crate for synchronization primitives.
use parking_lot::{Condvar, Mutex};
// Import the Arc and thread modules from the standard library for shared state and multi-threading.
use std::{sync::Arc, thread};

// Main function where the program starts execution.
fn main() {
    // Create a pair of a Mutex and a Condvar, wrapped in an Arc for shared ownership.
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    // Clone the Arc to create a new reference that can be moved into a new thread.
    let pair2 = pair.clone();

    // Spawn a new thread that will lock the Mutex, set the value to true, and then notify the Condvar.
    thread::spawn(move || {
        // Dereference the Arc to get a tuple of the Mutex and Condvar.
        let (ref lock, ref cvar) = &*pair2;
        // Lock the Mutex to gain exclusive access to the data.
        let mut started = lock.lock();
        // Set the value inside the Mutex to true.
        *started = true;
        // Notify the Condvar that the value has changed.
        cvar.notify_one();
    });

    // In the main thread, we lock the Mutex and wait for the value to change.
    let (ref lock, ref cvar) = &*pair;
    let mut started = lock.lock();

    // Check if the value is false, and if so, wait for a notification from the Condvar.
    if !*started {
        // Print the current value of `started` before waiting.
        println!("The value of `started` is: {}", *started);
        // Wait for a notification that the value has changed.
        cvar.wait(&mut started)
    };
    // After the wait, print the value of `started` again to show that it has changed.
    println!("The value of `started` is: {}", *started);
}
