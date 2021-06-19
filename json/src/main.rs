use std::process;

// Parse a simple JSON document
fn main() {
    let _01001_zip_data = r#"{ "_id" : "01001", "city" : "AGAWAM", "loc" : [ -72.622739, 42.070206 ], "pop" : 15338, "state" : "MA" }"#;
    println!("Parsing a JSON document: {}", _01001_zip_data);

    if let Err(e) = json::run(_01001_zip_data) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
