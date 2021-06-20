# notes

This is not an executable project but instead contains my notes about Rust.

## Reading Checklist

Crossing off sections of the book as I finish reading them.

**The Rust Programming Language**

<!-- This table of contents HTML taken from the Rust book webpage and edited to work best in a Markdown page -->
<ol style="list-style-type: none">
    <li style="list-style-type: none">The Rust Programming Language</li>
    <li style="list-style-type: none">Foreword</li>
    <li style="list-style-type: none">Introduction</li>
    <li style="list-style-type: none"><strong aria-hidden="true">1.</strong> Getting Started</li>
    <li>
        <ol style="list-style-type: none">
            <li style="list-style-type: none"><strong aria-hidden="true">1.1.</strong> Installation</li>
            <li style="list-style-type: none"><strong aria-hidden="true">1.2.</strong> Hello, World!</li>
            <li style="list-style-type: none"><strong aria-hidden="true">1.3.</strong> Hello, Cargo!</li>
        </ol>
    </li>
    <li style="list-style-type: none"><strong
            aria-hidden="true">2.</strong> Programming a Guessing Game
    </li>
    <li style="list-style-type: none"><strong
            aria-hidden="true">3.</strong> Common Programming Concepts
    </li>
    <li>
        <ol style="list-style-type: none">
            <li style="list-style-type: none"><strong aria-hidden="true">3.1.</strong>
                Variables and Mutability
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">3.2.</strong> Data Types
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">3.3.</strong> Functions
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">3.4.</strong> Comments
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">3.5.</strong> Control Flow
            </li>
        </ol>
    </li>
    <li style="list-style-type: none"><strong
            aria-hidden="true">4.</strong> Understanding Ownership
    </li>
    <li>
        <ol style="list-style-type: none">
            <li style="list-style-type: none"><strong
                    aria-hidden="true">4.1.</strong> What is Ownership?
            </li>
            <li style="list-style-type: none"><strong aria-hidden="true">4.2.</strong>
                References and Borrowing
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">4.3.</strong> The Slice Type
            </li>
        </ol>
    </li>
    <li style="list-style-type: none"><strong
            aria-hidden="true">5.</strong> Using Structs to Structure Related Data
    </li>
    <li>
        <ol style="list-style-type: none">
            <li style="list-style-type: none"><strong
                    aria-hidden="true">5.1.</strong> Defining and Instantiating Structs
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">5.2.</strong> An Example Program Using Structs
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">5.3.</strong> Method Syntax
            </li>
        </ol>
    </li>
    <li style="list-style-type: none"><strong
            aria-hidden="true">6.</strong> Enums and Pattern Matching
    </li>
    <li>
        <ol style="list-style-type: none">
            <li style="list-style-type: none"><strong
                    aria-hidden="true">6.1.</strong> Defining an Enum
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">6.2.</strong> The match Control Flow Operator
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">6.3.</strong> Concise Control Flow with if let
            </li>
        </ol>
    </li>
    <li style="list-style-type: none"><strong
            aria-hidden="true">7.</strong> Managing Growing Projects with Packages, Crates, and Modules
    </li>
    <li>
        <ol style="list-style-type: none">
            <li style="list-style-type: none"><strong
                    aria-hidden="true">7.1.</strong> Packages and Crates
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">7.2.</strong> Defining Modules to Control Scope and Privacy
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">7.3.</strong> Paths for Referring to an Item in the Module Tree
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">7.4.</strong> Bringing Paths Into Scope with the use Keyword
            </li>
            <li style="list-style-type: none"><strong aria-hidden="true">7.5.</strong>
                Separating Modules into Different Files
            </li>
        </ol>
    </li>
    <li style="list-style-type: none"><strong
            aria-hidden="true">8.</strong> Common Collections
    </li>
    <li>
        <ol style="list-style-type: none">
            <li style="list-style-type: none"><strong
                    aria-hidden="true">8.1.</strong> Storing Lists of Values with Vectors
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">8.2.</strong> Storing UTF-8 Encoded Text with Strings
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">8.3.</strong> Storing Keys with Associated Values in Hash Maps
            </li>
        </ol>
    </li>
    <li style="list-style-type: none"><strong
            aria-hidden="true">9.</strong> Error Handling
    </li>
    <li>
        <ol style="list-style-type: none">
            <li style="list-style-type: none"><strong aria-hidden="true">9.1.</strong>
                Unrecoverable Errors with panic!
            </li>
            <li style="list-style-type: none"><strong aria-hidden="true">9.2.</strong>
                Recoverable Errors with Result
            </li>
            <li style="list-style-type: none"><strong aria-hidden="true">9.3.</strong> To
                panic! or Not To panic!
            </li>
        </ol>
    </li>
    <li style="list-style-type: none"><strong aria-hidden="true">10.</strong>
        Generic Types, Traits, and Lifetimes
    </li>
    <li>
        <ol style="list-style-type: none">
            <li style="list-style-type: none"><strong
                    aria-hidden="true">10.1.</strong> Generic Data Types
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">10.2.</strong> Traits: Defining Shared Behavior
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">10.3.</strong> Validating References with Lifetimes
            </li>
        </ol>
    </li>
    <li style="list-style-type: none"><strong
            aria-hidden="true">11.</strong>
        Writing Automated Tests
    </li>
    <li>
        <ol style="list-style-type: none">
            <li style="list-style-type: none"><strong
                    aria-hidden="true">11.1.</strong> How to Write Tests
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">11.2.</strong> Controlling How Tests Are Run
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">11.3.</strong> Test Organization
            </li>
        </ol>
    </li>
    <li style="list-style-type: none"><strong
            aria-hidden="true">12.</strong> An I/O Project: Building a Command Line Program
    </li>
    <li>
        <ol style="list-style-type: none">
            <li style="list-style-type: none"><strong aria-hidden="true">12.1.</strong>
                Accepting Command Line Arguments
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">12.2.</strong> Reading a File
            </li>
            <li style="list-style-type: none"><strong aria-hidden="true">12.3.</strong>
                Refactoring to Improve Modularity and Error Handling
            </li>
            <li style="list-style-type: none"><strong aria-hidden="true">12.4.</strong>
                Developing the Library’s Functionality with Test Driven Development
            </li>
            <li style="list-style-type: none"><strong aria-hidden="true">12.5.</strong> Working
                with Environment Variables
            </li>
            <li style="list-style-type: none"><strong aria-hidden="true">12.6.</strong> Writing
                Error Messages to Standard Error Instead of Standard Output
            </li>
        </ol>
    </li>
    <li style="list-style-type: none"><strong
            aria-hidden="true">13.</strong> Functional Language Features: Iterators and Closures
    </li>
    <li>
        <ol style="list-style-type: none">
            <li style="list-style-type: none"><strong
                    aria-hidden="true">13.1.</strong> Closures: Anonymous Functions that Can Capture Their
                Environment
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">13.2.</strong> Processing a Series of Items with Iterators
            </li>
            <li style="list-style-type: none"><strong aria-hidden="true">13.3.</strong>
                Improving Our I/O Project
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">13.4.</strong> Comparing Performance: Loops vs. Iterators
            </li>
        </ol>
    </li>
    <li style="list-style-type: none"><strong
            aria-hidden="true">14.</strong> More about Cargo and Crates.io
    </li>
    <li>
        <ol style="list-style-type: none">
            <li style="list-style-type: none"><strong
                    aria-hidden="true">14.1.</strong> Customizing Builds with Release Profiles
            </li>
            <li style="list-style-type: none"><strong aria-hidden="true">14.2.</strong>
                Publishing a Crate to Crates.io
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">14.3.</strong> Cargo Workspaces
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">14.4.</strong> Installing Binaries from Crates.io with cargo install
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">14.5.</strong> Extending Cargo with Custom Commands
            </li>
        </ol>
    </li>
    <li style="list-style-type: none"><strong
            aria-hidden="true">15.</strong> Smart Pointers
    </li>
    <li>
        <ol style="list-style-type: none">
            <li style="list-style-type: none"><strong
                    aria-hidden="true">15.1.</strong> Using Box&lt;T&gt; to Point to Data on the Heap
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">15.2.</strong> Treating Smart Pointers Like Regular References with the
                Deref Trait
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">15.3.</strong> Running Code on Cleanup with the Drop Trait
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">15.4.</strong> Rc&lt;T&gt;, the Reference Counted Smart Pointer
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">15.5.</strong> RefCell&lt;T&gt; and the Interior Mutability Pattern
            </li>
            <li style="list-style-type: none"><strong aria-hidden="true">15.6.</strong>
                Reference Cycles Can Leak Memory
            </li>
        </ol>
    </li>
    <li style="list-style-type: none"><strong
            aria-hidden="true">16.</strong> Fearless Concurrency
    </li>
    <li>
        <ol style="list-style-type: none">
            <li style="list-style-type: none"><strong
                    aria-hidden="true">16.1.</strong> Using Threads to Run Code Simultaneously
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">16.2.</strong> Using Message Passing to Transfer Data Between Threads
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">16.3.</strong> Shared-State Concurrency
            </li>
            <li style="list-style-type: none"><strong aria-hidden="true">16.4.</strong>
                Extensible Concurrency with the Sync and Send Traits
            </li>
        </ol>
    </li>
    <li style="list-style-type: none"><strong
            aria-hidden="true">17.</strong> Object Oriented Programming Features of Rust
    </li>
    <li>
        <ol style="list-style-type: none">
            <li style="list-style-type: none"><strong
                    aria-hidden="true">17.1.</strong> Characteristics of Object-Oriented Languages
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">17.2.</strong> Using Trait Objects That Allow for Values of Different
                Types
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">17.3.</strong> Implementing an Object-Oriented Design Pattern
            </li>
        </ol>
    </li>
    <li style="list-style-type: none"><strong aria-hidden="true">18.</strong>
        Patterns and Matching
    </li>
    <li>
        <ol style="list-style-type: none">
            <li style="list-style-type: none"><strong aria-hidden="true">18.1.</strong> All the
                Places Patterns Can Be Used
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">18.2.</strong> Refutability: Whether a Pattern Might Fail to Match
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">18.3.</strong> Pattern Syntax
            </li>
        </ol>
    </li>
    <li style="list-style-type: none"><strong
            aria-hidden="true">19.</strong> Advanced Features
    </li>
    <li>
        <ol style="list-style-type: none">
            <li style="list-style-type: none"><strong
                    aria-hidden="true">19.1.</strong> Unsafe Rust
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">19.2.</strong> Advanced Traits
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">19.3.</strong> Advanced Types
            </li>
            <li style="list-style-type: none"><strong aria-hidden="true">19.4.</strong>
                Advanced Functions and Closures
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">19.5.</strong> Macros
            </li>
        </ol>
    </li>
    <li style="list-style-type: none"><strong
            aria-hidden="true">20.</strong> Final Project: Building a Multithreaded Web Server
    </li>
    <li>
        <ol style="list-style-type: none">
            <li style="list-style-type: none"><strong
                    aria-hidden="true">20.1.</strong> Building a Single-Threaded Web Server
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">20.2.</strong> Turning Our Single-Threaded Server into a Multithreaded
                Server
            </li>
            <li style="list-style-type: none"><strong aria-hidden="true">20.3.</strong>
                Graceful Shutdown and Cleanup
            </li>
        </ol>
    </li>
    <li style="list-style-type: none"><strong
            aria-hidden="true">21.</strong> Appendix
    </li>
    <li>
        <ol style="list-style-type: none">
            <li style="list-style-type: none"><strong
                    aria-hidden="true">21.1.</strong> A - Keywords
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">21.2.</strong> B - Operators and Symbols
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">21.3.</strong> C - Derivable Traits
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">21.4.</strong> D - Useful Development Tools
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">21.5.</strong> E - Editions
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">21.6.</strong> F - Translations of the Book
            </li>
            <li style="list-style-type: none"><strong
                    aria-hidden="true">21.7.</strong> G - How Rust is Made and “Nightly Rust”
            </li>
        </ol>
    </li>
</ol>

## Notes

Notes and quotes from [*The Rust Book Language* book](https://doc.rust-lang.org/stable/book/title-page.html).

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

* Quotes from [Chapter 12.3: *Refactoring to Improve Modularity and Error Handling*](https://doc.rust-lang.org/stable/book/ch12-03-improving-error-handling-and-modularity.html)
  > This Ok(()) syntax might look a bit strange at first, but using () like this is the idiomatic way to indicate that we’re calling run for its side effects only; it doesn’t return a value we need.

* Quotes from [Chapter 14: *More About Cargo and Crates.io*](https://doc.rust-lang.org/stable/book/ch14-00-more-about-cargo.html)
  > As your project develops, you might find that the library crate continues to get bigger and you want to split up your package further into multiple library crates. In this situation, Cargo offers a feature called workspaces that can help manage multiple related packages that are developed in tandem.

