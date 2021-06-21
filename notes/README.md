# notes

This is not an executable project but instead contains my notes about Rust.

I'm following [*The Rust Programming Language*](https://doc.rust-lang.org/stable/book) book by Carol Nichols and Steve Klabnik
which is affectionately referred to as "The Book" in the Rust community.

I'm also using [*The Rust Reference*](https://doc.rust-lang.org/reference/introduction.html) book too.

## Reading Checklist

Crossing off sections of *The Rust Programming Language* book as I finish reading them.

**The Rust Programming Language**

- [x] The Rust Programming Language
- [x] Foreword
- [x] Introduction
- [x] 1\. Getting Started
   - [x] 1.1. Installation
   - [x] 1.2. Hello, World!
   - [x] 1.3. Hello, Cargo!
- [x] 2\. Programming a Guessing Game
- [x] 3\. Common Programming Concepts
   - [x] 3.1. Variables and Mutability
   - [x] 3.2. Data Types
   - [x] 3.3. Functions
   - [x] 3.4. Comments
   - [x] 3.5. Control Flow
- [x] 4\. Understanding Ownership
   - [x] 4.1. What is Ownership?
   - [x] 4.2. References and Borrowing
   - [x] 4.3. The Slice Type
- [x] 5\. Using Structs to Structure Related Data
   - [x] 5.1. Defining and Instantiating Structs
   - [x] 5.2. An Example Program Using Structs
   - [x] 5.3. Method Syntax
- [ ] 6\. Enums and Pattern Matching
   - [ ] 6.1. Defining an Enum
   - [ ] 6.2. The match Control Flow Operator
   - [ ] 6.3. Concise Control Flow with if let
- [ ] 7\. Managing Growing Projects with Packages, Crates, and Modules
   - [x] 7.1. Packages and Crates
   - [x] 7.2. Defining Modules to Control Scope and Privacy
   - [ ] 7.3. Paths for Referring to an Item in the Module Tree
   - [ ] 7.4. Bringing Paths Into Scope with the use Keyword
   - [x] 7.5. Separating Modules into Different Files
- [ ] 8\. Common Collections
   - [ ] 8.1. Storing Lists of Values with Vectors
   - [ ] 8.2. Storing UTF-8 Encoded Text with Strings
   - [ ] 8.3. Storing Keys with Associated Values in Hash Maps
- [ ] 9\. Error Handling
   - [ ] 9.1. Unrecoverable Errors with panic!
   - [ ] 9.2. Recoverable Errors with Result
   - [ ] 9.3. To panic! or Not To panic!
- [ ] 10\. Generic Types, Traits, and Lifetimes
    - [x] 10.1. Generic Data Types
    - [x] 10.2. Traits: Defining Shared Behavior
    - [ ] 10.3. Validating References with Lifetimes
- [ ] 11\. Writing Automated Tests
    - [ ] 11.1. How to Write Tests
    - [ ] 11.2. Controlling How Tests Are Run
    - [ ] 11.3. Test Organization
- [ ] 12\. An I/O Project: Building a Command Line Program
    - [x] 12.1. Accepting Command Line Arguments
    - [x] 12.2. Reading a File
    - [x] 12.3. Refactoring to Improve Modularity and Error Handling
    - [ ] 12.4. Developing the Library’s Functionality with Test Driven Development
    - [ ] 12.5. Working with Environment Variables
    - [ ] 12.6. Writing Error Messages to Standard Error Instead of Standard Output
- [ ] 13\. Functional Language Features: Iterators and Closures
    - [ ] 13.1. Closures: Anonymous Functions that Can Capture Their Environment
    - [x] 13.2. Processing a Series of Items with Iterators
    - [ ] 13.3. Improving Our I/O Project
    - [ ] 13.4. Comparing Performance: Loops vs. Iterators
- [ ] 14\. More about Cargo and Crates.io
    - [ ] 14.1. Customizing Builds with Release Profiles
    - [ ] 14.2. Publishing a Crate to Crates.io
    - [ ] 14.3. Cargo Workspaces
    - [ ] 14.4. Installing Binaries from Crates.io with cargo install
    - [ ] 14.5. Extending Cargo with Custom Commands
- [ ] 15\. Smart Pointers
    - [ ] 15.1. Using Box<T> to Point to Data on the Heap
    - [ ] 15.2. Treating Smart Pointers Like Regular References with the Deref Trait
    - [ ] 15.3. Running Code on Cleanup with the Drop Trait
    - [ ] 15.4. Rc<T>, the Reference Counted Smart Pointer
    - [ ] 15.5. RefCell<T> and the Interior Mutability Pattern
    - [ ] 15.6. Reference Cycles Can Leak Memory
- [ ] 16\. Fearless Concurrency
    - [ ] 16.1. Using Threads to Run Code Simultaneously
    - [ ] 16.2. Using Message Passing to Transfer Data Between Threads
    - [ ] 16.3. Shared-State Concurrency
    - [ ] 16.4. Extensible Concurrency with the Sync and Send Traits
- [ ] 17\. Object Oriented Programming Features of Rust
    - [ ] 17.1. Characteristics of Object-Oriented Languages
    - [ ] 17.2. Using Trait Objects That Allow for Values of Different Types
    - [ ] 17.3. Implementing an Object-Oriented Design Pattern
- [ ] 18\. Patterns and Matching
    - [ ] 18.1. All the Places Patterns Can Be Used
    - [ ] 18.2. Refutability: Whether a Pattern Might Fail to Match
    - [ ] 18.3. Pattern Syntax
- [ ] 19\. Advanced Features
    - [ ] 19.1. Unsafe Rust
    - [ ] 19.2. Advanced Traits
    - [ ] 19.3. Advanced Types
    - [ ] 19.4. Advanced Functions and Closures
    - [ ] 19.5. Macros
- [ ] 20\. Final Project: Building a Multithreaded Web Server
    - [ ] 20.1. Building a Single-Threaded Web Server
    - [ ] 20.2. Turning Our Single-Threaded Server into a Multithreaded Server
    - [ ] 20.3. Graceful Shutdown and Cleanup
- [ ] 21\. Appendix
    - [ ] 21.1. A - Keywords
    - [ ] 21.2. B - Operators and Symbols
    - [ ] 21.3. C - Derivable Traits
    - [ ] 21.4. D - Useful Development Tools
    - [ ] 21.5. E - Editions
    - [ ] 21.6. F - Translations of the Book
    - [ ] 21.7. G - How Rust is Made and “Nightly Rust”

## Notes from *The Book*

Notes and quotes from [*The Rust Programming Language*](https://doc.rust-lang.org/stable/book).

* Quotes from [Chapter 3.3: *Function Bodies Contain Statements and Expressions*](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html#function-bodies-contain-statements-and-expressions)
  > Rust is an expression-based language

  > Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value.
* The compiler error messages have advice to fix code that doesn't compile. This is nice!
* Quotes from [Chapter 3.5: *Control Flow*](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#control-flow)
  > Blocks of code associated with the conditions in if expressions are sometimes called arms

  > When we run this program, we’ll see again! printed over and over continuously until we stop the program manually. Most terminals support a keyboard shortcut, ctrl-c, to interrupt a program that is stuck in a continual loop. Give it a try:...
    * This guide makes Rust approachable for beginners. Is Rust more approachable than other programming languages??
* Quotes from [Chapter 4.1: *What Is Ownership?*](https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html)
  > A scope is the range within a program for which an item is valid.

  > The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless the data has been moved to be owned by another variable.
* I feel like it's going to be tricky to remember that tuples implement the Copy trait if and only if all of the component
  types implement Copy. Will I get 'borrow of moved value' compile errors frequently with tuples? Or maybe I'll just get
  that all the time for all sorts of reasons.
* Quotes from [Chapter 4.2: *References and Borrowing*](https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html)
  > We call having references as function parameters borrowing. As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back.

  > Dangling References...the compiler will ensure that the data will not go out of scope before the reference to the data does.
* Quotes from [Chapter 5.3: *Method Syntax*](https://doc.rust-lang.org/stable/book/ch05-03-method-syntax.html)
  > We’ve put all the things we can do with an instance of a type in one impl block rather than making future users of our code search for capabilities of Rectangle in various places in the library we provide.

  > The fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice.

* Quotes from [Chapter 7: *Managing Growing Projects with Packages, Crates, and Modules*](https://doc.rust-lang.org/stable/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
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

* Quotes from [Chapter 10: *Generic Types, Traits, and Lifetimes*](https://doc.rust-lang.org/stable/book/ch10-00-generics.html)
  * [10.1: *Generic Data Types*](https://doc.rust-lang.org/stable/book/ch10-01-syntax.html#generic-data-types)
    > Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.
    
    > Because Rust compiles generic code into code that specifies the type in each instance, we pay no runtime cost for using generics. When the code runs, it performs just as it would if we had duplicated each definition by hand. The process of monomorphization makes Rust’s generics extremely efficient at runtime.

  * [10.2: *Traits: Defining Shared Behavior*](https://doc.rust-lang.org/stable/book/ch10-02-traits.html)
    > One restriction to note with trait implementations is that we can implement a trait on a type only if either the trait or the type is local to our crate.
    > 
    > ...
    > 
    > This restriction is part of a property of programs called *coherence*, and more specifically the *orphan rule*, so named because the parent type is not present. This rule ensures that other people’s code can’t break your code and vice versa. Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t know which implementation to use.
    
    > The `impl Trait` syntax works for straightforward cases but is actually syntax sugar for a longer form, which is called a *trait bound*
  
    > Rust has alternate syntax for specifying trait bounds inside a `where` clause after the function signature.

    > Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations and are extensively used in the Rust standard library.

* Quotes from [Chapter 12.3: *Refactoring to Improve Modularity and Error Handling*](https://doc.rust-lang.org/stable/book/ch12-03-improving-error-handling-and-modularity.html)
  > This Ok(()) syntax might look a bit strange at first, but using () like this is the idiomatic way to indicate that we’re calling run for its side effects only; it doesn’t return a value we need.

* Quotes from [Chapter 14: *More About Cargo and Crates.io*](https://doc.rust-lang.org/stable/book/ch14-00-more-about-cargo.html)
  > As your project develops, you might find that the library crate continues to get bigger and you want to split up your package further into multiple library crates. In this situation, Cargo offers a feature called workspaces that can help manage multiple related packages that are developed in tandem.

## Questions

* What are instances of structs called? Are they called objects? Or, is this not correct and I should not be thinking in
  the object-oriented mindset like with Java/JavaScript etc. For that matter, can I have an instance of things other than
  structs? I think I can have instances of traits, and some other things I suspect. What are these things called? Should
  we always use the long form, "this is an instance of XYZ" or can we call them "objects"? 
* I don't grok the full range of syntax sugar when it comes to types that implement the `Copy` trait. I don't even know
  how to express this thought so I'll leave it at that. 
* I'm a fan of Rust's `Self` keyword. If Java had this, for example, we wouldn't have to type out the whole class name in the
  constructor definitions. This is especially annoying when the class name is very long and/or there are many constructor
  overloads.
* How does the automatic dereferencing syntax sugar (rather, compilation sugar magic?) work again? For example, in a function
  whose signature includes `&self`, you can author code like `self.xyz` (assuming there is a field `xyz`) and Rust knows
  what you mean and you don't need to do `*self.xyz` or whatever. 
