use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Task {
    id: u32,
    payload: String,
}

fn create_task(id: u32, payload: &str) -> Task {
    Task {
        id,
        payload: payload.to_string(),
    }
}

struct Worker;

impl Worker {
    fn process_task(task: Task) -> String {
        println!("Processing task {:?}", task);
        thread::sleep(Duration::from_secs(1)); // Simulate task processing
        format!("Task {} processed", task.id)
    }
}

fn main() {
    let (tx, rx): (Sender<Task>, Receiver<Task>) = mpsc::channel();
    let (result_tx, result_rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    // Create a worker thread
    thread::spawn(move || {
        while let Ok(task) = rx.recv() {
            let result = Worker::process_task(task);
            result_tx.send(result).unwrap();
        }
    });

    // Create tasks and send them to the worker
    let tasks = vec![
        create_task(1, "Task 1"),
        create_task(2, "Task 2"),
        create_task(3, "Task 3"),
    ];

    for task in tasks {
        tx.send(task).unwrap();
    }

    drop(tx); // Close the sending end of the channel

    // Receive and print completion messages
    for received in result_rx {
        println!("{}", received);
    }
}
