# rust-playground

📚 Learning and exploring the Rust programming language.

To start, I'm following the [*The Rust Programming Language*](https://doc.rust-lang.org/stable/book) which is
affectionately referred to as "The Book" in the Rust community.

## Standalone sub-projects

This repository illustrates different concepts, patterns and examples via standalone sub-projects. Each sub-project is
completely independent of the others and do not depend on the root project. This _standalone sub-project constraint_
forces the sub-projects to be complete and maximizes the reader's chances of successfully running, understanding, and
re-using the code.

The sub-projects include:

### `guessing_game/`

Verbatim copy of the [*Programming a Guessing Game* chapter](https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html).

See the README in [`guessing_game/`](guessing_game/).

### `read_file/`

A simple Rust program to read a file and parse JSON using the popular Rust JSON library called [*Serde*](https://github.com/serde-rs/json).  

See the README in [`read_file/`](read_file/).

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

* Quotes from [*Managing Growing Projects with Packages, Crates, and Modules*](https://doc.rust-lang.org/stable/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
  > Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the module system, include:
  >
  >   * **Packages**: A Cargo feature that lets you build, test, and share crates
  >   * **Crates**: A tree of modules that produces a library or executable
  >   * **Modules** and use: Let you control the organization, scope, and privacy of paths
  >   * **Paths**: A way of naming an item, such as a struct, function, or module  

  > ...Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package
    * Confusing sentence. What do package names have to do with the idea that the file `src/main.rs` is a Cargo convention?
      Or, is the file `main.rs` itself literally a crate? Can a crate be made up of more files (I mean I'm 99% sure yes,
      I should just keep reading).
  > If a package contains src/main.rs and src/lib.rs, it has two crates: a library and a binary, both with the same name as the package.
    * The mental model conflation factor is high. We have one entity--the package--which in this case we can use the
      example name 'my-project' which is also the same name as the project (*project* is not a Rust-specific concept to
      be fair but even this doc chapter uses it). On top of that we have two crates (one binary, one library) with that
      same name. In sum:
        * Project named "my-project"
          * You can think of the root directory itself as the project. This is a universal, non-Rust concept. 
        * Package named "my-project"
          * Think of the `Cargo.toml` file as the identifying thing of the package (I think?). 
        * Binary crate named "my-project"
          * Think of the file `src/main.rs` as the identifying thing of this crate. 
        * Library crate named "my-project"
          * Think of the file `src/lib.rs` as the identifying thing of this crate.
      
      Phew, now I understand it well after writing all that.
    
  > Using a semicolon after mod front_of_house rather than using a block tells Rust to load the contents of the module from another file with the same name as the module.

  > The mod keyword declares modules, and Rust looks in a file with the same name as the module for the code that goes into that module. 

* Quotes from [*Refactoring to Improve Modularity and Error Handling*](https://doc.rust-lang.org/stable/book/ch12-03-improving-error-handling-and-modularity.html)
  > This Ok(()) syntax might look a bit strange at first, but using () like this is the idiomatic way to indicate that we’re calling run for its side effects only; it doesn’t return a value we need.

## Wish list

General clean ups, TODOs and things I wish to implement for this project:

* Create an error handling sub-project
  * E.g. `Result/Ok/Err` and the `?` special thing
* Create an argument parsing sub-project
* Create an async sub-project
* Create a sub-project that shells out to another process
