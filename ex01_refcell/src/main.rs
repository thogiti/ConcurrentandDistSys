// Import the RefCell type from the standard library for interior mutability.
use std::cell::RefCell;

// Define a simple function that returns true.
fn f1() -> bool {
    true
}

// Define another function that returns the negation of f1.
fn f2() -> bool {
    !f1()
}

// Entry point of the program.
fn main() {
    // Create two RefCells, wrapping integers  5 and  666.
    let v1 = RefCell::new(5);
    let v2 = RefCell::new(666);

    // Conditionally borrow either v1 or v2 based on the return value of f1.
    // Since f1 returns true, v1 is borrowed immutably.
    let p1 = if f1() { &v1 } else { &v2 }.try_borrow().unwrap();

    // Check the result of f2. Since f2 returns false, this block is skipped.
    if f2() {
        // This block would never execute due to the condition above.
        let mut p2 = v1.try_borrow_mut().unwrap();
        *p2 = 35;
        println!("p2 : {}", *p2);
    }

    // Print the value referenced by p1, which should be  5 since v1 was borrowed.
    println!("p1 : {}", *p1);
}
