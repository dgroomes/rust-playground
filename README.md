# rust-playground

📚 Learning and exploring the Rust programming language.

To start, I'm following the [*The Rust Programming Language*](https://doc.rust-lang.org/stable/book) which is
affectionately referred to as "The Book" in the Rust community.

## Notes

Some notes.

* Quotes from [*Function Bodies Contain Statements and Expressions*](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html#function-bodies-contain-statements-and-expressions)
  > Rust is an expression-based language
  
  > Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value.
* The compiler error messages have advice to fix code that doesn't compile. This is nice!
* Quotes from [*Control Flow*](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#control-flow)
  > Blocks of code associated with the conditions in if expressions are sometimes called arms
  
  > When we run this program, we’ll see again! printed over and over continuously until we stop the program manually. Most terminals support a keyboard shortcut, ctrl-c, to interrupt a program that is stuck in a continual loop. Give it a try:...
    * This guide makes Rust approachable for beginners. Is Rust more approachable than other programming languages??
* Quotes from [*What Is Ownership?*](https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html)
  > A scope is the range within a program for which an item is valid.

  > The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless the data has been moved to be owned by another variable. 
* I feel like it's going to be tricky to remember that tuples implement the Copy trait if and only if all of the component
  types implement Copy. Will I get 'borrow of moved value' compile errors frequently with tuples? Or maybe I'll just get
  that all the time for all sorts of reasons. 
* Quotes from [*References and Borrowing*](https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html)
  > We call having references as function parameters borrowing. As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back.
  
  > Dangling References...the compiler will ensure that the data will not go out of scope before the reference to the data does.
* Quotes from [*Method Syntax*](https://doc.rust-lang.org/stable/book/ch05-03-method-syntax.html)
  > We’ve put all the things we can do with an instance of a type in one impl block rather than making future users of our code search for capabilities of Rectangle in various places in the library we provide.

  > The fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice.
* Quotes from [*Refactoring to Improve Modularity and Error Handling*](https://doc.rust-lang.org/stable/book/ch12-03-improving-error-handling-and-modularity.html)
  > This Ok(()) syntax might look a bit strange at first, but using () like this is the idiomatic way to indicate that we’re calling run for its side effects only; it doesn’t return a value we need.
