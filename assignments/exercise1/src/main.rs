use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

// Define the Task struct
#[derive(Debug, Clone)]
struct Task {
    id: u32,
    payload: String,
}

// Function to create a new Task
fn create_task(id: u32, payload: &str) -> Task {
    Task {
        id,
        payload: payload.to_string(),
    }
}

// Define the Worker struct
#[derive(Debug)]
struct Worker {
    id: u32,
}

// Function to create a new Worker
fn create_worker(id: u32) -> Worker {
    Worker { id }
}

// Implement the process_task method for Worker
impl Worker {
    fn process_task(&self, task: Task) -> String {
        // Simulate task processing with a sleep
        thread::sleep(Duration::from_secs(1));
        format!("Worker {} processed task {}", self.id, task.id)
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));
    let (result_tx, result_rx) = mpsc::channel();

    // Create workers
    let workers = vec![create_worker(1), create_worker(2), create_worker(3)];

    // Spawn threads for each worker
    let mut handles = Vec::new();
    for worker in workers {
        let rx = Arc::clone(&rx);
        let result_tx = result_tx.clone();
        let handle = thread::spawn(move || {
            while let Ok(task) = rx.lock().unwrap().recv() {
                let result = worker.process_task(task);
                result_tx.send(result).unwrap();
            }
        });
        handles.push(handle);
    }

    // Create and send tasks
    let tasks = vec![
        create_task(1, "Task 1"),
        create_task(2, "Task 2"),
        create_task(3, "Task 3"),
        create_task(4, "Task 4"),
        create_task(5, "Task 5"),
        create_task(6, "Task 6"),
        create_task(7, "Task 7"),
        create_task(8, "Task 8"),
    ];
    for task in tasks {
        tx.send(task).unwrap();
    }

    // Drop the original sender to ensure threads exit once all tasks are processed
    drop(tx);

    // Collect and print results
    for res in result_rx {
        println!("Received message: {:?}", res);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}
