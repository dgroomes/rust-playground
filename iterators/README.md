# iterators

A simple program that showcases "iterators" in Rust.

## Instructions

Follow these instructions to build and run the program:

1. Build and run:
   * `cargo run`
2. Altogether, it should look something like this:
    ```text
    $ cargo run
    Compiling iterators v0.1.0 (/Users/davidgroomes/repos/personal/rust-playground/iterators)
    Finished dev [unoptimized + debuginfo] target(s) in 0.77s
    Running `target/debug/iterators`
    
    Showing different ways to iterate over elements.
    
    
    Iterating using the 'for in' syntax...
    Found message: 'Hello'
    Found message: 'there,'
    Found message: 'world!'
    
    Iterating 'by hand'...
    Found message: 'Hello'
    Found message: 'there,'
    Found message: 'world!'
    ```

## Wish list

General clean ups, TODOs and things I wish to implement for this project:

* ABANDONED Create a custom iterator.
  * I abandoned this because I started reading the iterator code and hit a macro. No idea what's going on. I'll visit
    macros at a later date. There's so much unsafe code too in the iterator implementation. Related code snippets linked
    below.
  * <https://github.com/rust-lang/rust/blob/f2661cfe341f88bea919daf52a07015dceaf7a6a/library/core/src/slice/iter.rs#L134>
  * <https://github.com/rust-lang/rust/blob/f2661cfe341f88bea919daf52a07015dceaf7a6a/library/core/src/slice/iter/macros.rs#L41>
  

## Reference

* [*The Rust Programming Language: Chapter 13. Functional Language Features: Iterators and Closures*](https://doc.rust-lang.org/stable/book/ch13-00-functional-features.html)
