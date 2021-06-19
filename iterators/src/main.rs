use std::process;

// Illustrating Rust's iterators.
fn main() {
    println!("Showing different ways to iterate over elements.\n");

    if let Err(e) = iterators::run() {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
