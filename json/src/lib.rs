use std::error::Error;
use serde::{Deserialize, Serialize};
use std::fmt::{Formatter};

pub fn run(json: &str) -> Result<(), Box<dyn Error>> {
    let zip_area: ZipArea = serde_json::from_str(&json).expect("Failed to deserialize JSON");
    println!("Deserialized to a struct: {}", &zip_area);

    let json_again = serde_json::to_string_pretty(&zip_area).expect("Failed to serialize JSON");
    println!("Serialized to JSON again (but pretty printed and without all of the original fields):\n{}", &json_again);

    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
struct ZipArea {
    city: String,
    state: String,
    pop: u64,
}

// This is Rust's version of the "toString" idiom common in most languages.
// See the docs: https://doc.rust-lang.org/std/fmt/trait.Display.html
impl std::fmt::Display for ZipArea {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "city={}, state={}, pop={}", self.city, self.state, self.pop)
    }
}
