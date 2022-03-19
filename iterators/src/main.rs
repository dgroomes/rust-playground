use std::process;

// Illustrating Rust's iterators.
fn main() {
    println!("\nShowing different ways to iterate over elements.\n\n");

    if let Err(e) = iterators::run() {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
