use std::error::Error;
use std::slice::Iter;

pub fn run() -> Result<(), Box<dyn Error>> {
    let messages = vec!["Hello", "there,", "world!"];

    println!("Iterating using the 'for in' syntax...");
    iterate(&messages);
    println!();

    println!("Iterating 'by hand'...");
    iterate_by_hand(&messages);

    Ok(())
}

fn iterate(messages: &Vec<&str>) {
    for message in messages {
        println!("Found message: '{}'", message)
    }
}

// Iterate "by hand" over the values produced by an iterator. Normally, you would use the "for in..."
// syntax sugar instead of doing this. But this is for educational purposes.
fn iterate_by_hand(messages: &Vec<&str>) {
    let mut iter: Iter<&str> = messages.iter();

    loop {
        let option: Option<&&str> = iter.next();
        match option {
            None => {
                // When the iterator returns a None then it is the end of the iterator. I like this
                // pattern compared to Java's "hasNext" way to check if an iterator is done. I like
                // the Java way too. It's just a different idiom.
                break;
            }
            Some(message) => {
                println!("Found message: '{}'", message);
            }
        }
    }
}

#[allow(dead_code)]
fn iterate_custom_iterator() {
    todo!()
}

