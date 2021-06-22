# notes

This is not an executable project but instead contains my notes about Rust.

I'm following [*The Rust Programming Language*](https://doc.rust-lang.org/stable/book) book by Carol Nichols and Steve Klabnik
which is affectionately referred to as "The Book" in the Rust community.

I'm also using [*The Rust Reference*](https://doc.rust-lang.org/reference/introduction.html) book too.

# Reading Checklist

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
- [x] 6\. Enums and Pattern Matching
   - [x] 6.1. Defining an Enum
   - [x] 6.2. The match Control Flow Operator
   - [x] 6.3. Concise Control Flow with if let
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
- [x] 10\. Generic Types, Traits, and Lifetimes
    - [x] 10.1. Generic Data Types
    - [x] 10.2. Traits: Defining Shared Behavior
    - [x] 10.3. Validating References with Lifetimes
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
    - [x] 13.1. Closures: Anonymous Functions that Can Capture Their Environment
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
- [x] 17\. Object Oriented Programming Features of Rust
    - [x] 17.1. Characteristics of Object-Oriented Languages
    - [x] 17.2. Using Trait Objects That Allow for Values of Different Types
    - [x] 17.3. Implementing an Object-Oriented Design Pattern
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

Notes and quotes from [*The Rust Programming Language*](https://doc.rust-lang.org/stable/book) organized by chapters and
sub-chapters.

### [Chapter 3: *Common Programming Concepts*](https://doc.rust-lang.org/stable/book/ch03-00-common-programming-concepts.html)

#### [3.3: *Functions*](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html)

* > Rust is an expression-based language
* > Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value.
* The compiler error messages have advice to fix code that doesn't compile. This is nice!

#### [3.5: *Control Flow*](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#control-flow)

* > Blocks of code associated with the conditions in if expressions are sometimes called *arms*
* > When we run this program, we’ll see again! printed over and over continuously until we stop the program manually. Most terminals support a keyboard shortcut, ctrl-c, to interrupt a program that is stuck in a continual loop. Give it a try:...
    * This guide makes Rust approachable for beginners. Is Rust more approachable than other programming languages?? UPDATE:
      no it is not! Although this book does an excellent job at teach Rust, it is ultimately a hard-mode language.

### [Chapter 4: *Understanding Ownership*](https://doc.rust-lang.org/stable/book/ch04-00-understanding-ownership.html)

#### [4.1: *What Is Ownership?*](https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html)

* > A scope is the range within a program for which an item is valid.
* > The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless the data has been moved to be owned by another variable.
* I feel like it's going to be tricky to remember that tuples implement the Copy trait if and only if all of the component
  types implement Copy. Will I get 'borrow of moved value' compile errors frequently with tuples? Or maybe I'll just get
  that all the time for all sorts of reasons.

#### [4.2: *References and Borrowing*](https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html)

* > We call having references as function parameters borrowing. As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back.
* > Dangling References...the compiler will ensure that the data will not go out of scope before the reference to the data does.

### [Chapter 5: *Using Structs to Structure Related Data*](https://doc.rust-lang.org/stable/book/ch05-00-structs.html)

#### [5.3: *Method Syntax*](https://doc.rust-lang.org/stable/book/ch05-03-method-syntax.html)

* > We’ve put all the things we can do with an instance of a type in one impl block rather than making future users of our code search for capabilities of Rectangle in various places in the library we provide.
* > The fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice.

### [Chapter 6: *Enums and Pattern Matching*](https://doc.rust-lang.org/stable/book/ch06-00-enums.html)

* > Enums are a feature in many languages, but their capabilities differ in each language. Rust’s enums are most similar to algebraic data types in functional languages, such as F#, OCaml, and Haskell.
  
#### [6.1: *Defining an Enum*](https://doc.rust-lang.org/stable/book/ch06-01-defining-an-enum.html)

* Rust enums are very different from Java enums. In Rust, you can have different instances of an enum type. Rust's enums
  have a similar type system as [Java's sealed classes](https://openjdk.java.net/jeps/409) (which are coming in Java 17!).
  Java's sealed classes will enable more pattern matching in Java. In this way, Java is becoming more like Rust (of course,
  Rust didn't invent pattern matching but for the sake of understanding Java and Rust by comparing and contrasting them,
  this is a useful characterization).
* The values (sub-types?) of a Rust enum are called *variants*.
* > The `Option` type is used in many places because it encodes the very common scenario in which a value could be something or it could be nothing.
* > In other words, you have to convert an `Option<T>` to a `T` before you can perform `T` operations with it. Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is.

#### [6.3: *Concise Control Flow with `if let`*](https://doc.rust-lang.org/stable/book/ch06-03-if-let.html)

* > Using `if let` means less typing, less indentation, and less boilerplate code. However, you lose the exhaustive checking that `match` enforces. Choosing between `match` and `if let` depends on what you’re doing in your particular situation and whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.
  
### [Chapter 7: *Managing Growing Projects with Packages, Crates, and Modules*](https://doc.rust-lang.org/stable/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

* > Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the module system, include:
  >
  >   * **Packages**: A Cargo feature that lets you build, test, and share crates
  >   * **Crates**: A tree of modules that produces a library or executable
  >   * **Modules** and use: Let you control the organization, scope, and privacy of paths
  >   * **Paths**: A way of naming an item, such as a struct, function, or module

* > ...Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package
   * Confusing sentence. What do package names have to do with the idea that the file `src/main.rs` is a Cargo convention?
     Or, is the file `main.rs` itself literally a crate? Can a crate be made up of more files (I mean I'm 99% sure yes,
     I should just keep reading).
* > If a package contains src/main.rs and src/lib.rs, it has two crates: a library and a binary, both with the same name as the package.
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
* > Using a semicolon after mod front_of_house rather than using a block tells Rust to load the contents of the module from another file with the same name as the module.
* > The mod keyword declares modules, and Rust looks in a file with the same name as the module for the code that goes into that module.

### [Chapter 10: *Generic Types, Traits, and Lifetimes*](https://doc.rust-lang.org/stable/book/ch10-00-generics.html)

#### [10.1: *Generic Data Types*](https://doc.rust-lang.org/stable/book/ch10-01-syntax.html#generic-data-types)

* > Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.
* > Because Rust compiles generic code into code that specifies the type in each instance, we pay no runtime cost for using generics. When the code runs, it performs just as it would if we had duplicated each definition by hand. The process of monomorphization makes Rust’s generics extremely efficient at runtime.
  
#### [10.2: *Traits: Defining Shared Behavior*](https://doc.rust-lang.org/stable/book/ch10-02-traits.html)

* > One restriction to note with trait implementations is that we can implement a trait on a type only if either the trait or the type is local to our crate.
  > 
  > ...
  > 
  > This restriction is part of a property of programs called *coherence*, and more specifically the *orphan rule*, so named because the parent type is not present. This rule ensures that other people’s code can’t break your code and vice versa. Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t know which implementation to use.
* > The `impl Trait` syntax works for straightforward cases but is actually syntax sugar for a longer form, which is called a *trait bound*
* > Rust has alternate syntax for specifying trait bounds inside a `where` clause after the function signature.
* > Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations and are extensively used in the Rust standard library.

#### [10.3: *Validating References with Lifetimes*](https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html)

* > The concept of lifetimes is somewhat different from tools in other programming languages, arguably making lifetimes Rust’s most distinctive feature.
   * The [illustration](https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html#the-borrow-checker) of the borrow
      checker using in-line code comments and the "won't compile crustacean" is simple and effective. Brilliant.
* >  Lifetime annotations describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.
   * I don't quite get this yet.
* > Remember, when we specify the lifetime parameters in this function signature, we’re not changing the lifetimes of any values passed in or returned.
   * I don't understand this. I'll try to remember this as a fact even though I don't get it. It seems like we *are*
     extending the lifetime of the 'xyz' string value to last as long as the lifetime of the return value of the `longest`
     function. For example, if the program does a ton of work between the invocation of `longest` and when its return
     value is actually used, we are holding onto the memory used for the 'xyz' value when we could have freed the memory
     long ago.
* > Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions. Once they’re connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers or otherwise violate memory safety.
* > Lifetimes on function or method parameters are called *input lifetimes*, and lifetimes on return values are called *output lifetimes*.
* > lifetimes are a type of generic
* This was the hardest section of I've read so far. 

### [Chapter 12: *An I/O Project: Building a Command Line Program*](https://doc.rust-lang.org/stable/book/ch12-00-an-io-project.html)

#### [12.3: *Refactoring to Improve Modularity and Error Handling*](https://doc.rust-lang.org/stable/book/ch12-03-improving-error-handling-and-modularity.html)

* > This Ok(()) syntax might look a bit strange at first, but using () like this is the idiomatic way to indicate that we’re calling run for its side effects only; it doesn’t return a value we need.

### [Chapter 13: *Functional Language Features: Iterators and Closures*](https://doc.rust-lang.org/stable/book/ch13-00-functional-features.html)

#### [13.1: *Closures: Anonymous Functions that Can Capture Their Environment*](https://doc.rust-lang.org/stable/book/ch13-01-closures.html)

* > Unlike functions, closures can capture values from the scope in which they’re defined.
   * Well said.
* Wow, the type inference is strong in Rust. The type of the closure declaration `let add_one_v4 = |x| x + 1 ;` is not
  even known until the closure is invoked later in the function! Wow, the Rust compiler can accommodate a large context
  of stuff (lines of code) to do type inference. Cool stuff.
* > Each closure instance has its own unique anonymous type: that is, even if two closures have the same signature, their types are still considered different.
* > When you create a closure, Rust infers which trait to use based on how the closure uses the values from the environment.
* I can't quite follow all the details of the `Fn` traits or *`move` closures* (scary!). But that's ok. 

### [Chapter 14: *More About Cargo and Crates.io*](https://doc.rust-lang.org/stable/book/ch14-00-more-about-cargo.html)

* > As your project develops, you might find that the library crate continues to get bigger and you want to split up your package further into multiple library crates. In this situation, Cargo offers a feature called workspaces that can help manage multiple related packages that are developed in tandem.

### [Chapter 17: *Object Oriented Programming Features of Rust*](https://doc.rust-lang.org/stable/book/ch17-00-oop.html)

#### [17.2: *Using Trait Objects That Allow for Values of Different Types*](https://doc.rust-lang.org/stable/book/ch17-02-trait-objects.html)
  
* > A generic type parameter can only be substituted with one concrete type at a time, whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime.
* > When we use trait objects, Rust must use dynamic dispatch. The compiler doesn’t know all the types that might be used with the code that is using trait objects, so it doesn’t know which method implemented on which type to call.

#### [17.3: *Implementing an Object-Oriented Design Pattern*](https://doc.rust-lang.org/stable/book/ch17-03-oo-design-patterns.html)
    
* I don't totally get it, but I think this quote and the accompanying code snippet are an example of the acrobatics
  you need to do in Rust due to a combination of Rust not allowing nulls and of Rust's ownership feature:
   * > We need to set `state` to `None` temporarily rather than setting it directly with code like `self.state = self.state.request_review();` to get ownership of the `state` value. This ensures `Post` can’t use the old `state` value after we’ve transformed it into a new state.
   * ```
     pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
             self.state = Some(s.request_review())
        }
     }
     ```
* > Now we can start seeing the advantages of the state pattern: the `request_review` method on `Post` is the same no matter its `state` value. Each state is responsible for its own rules.
    
## Questions

* What are instances of structs called? Are they called objects? Or, is this not correct and I should not be thinking in
  the object-oriented mindset like with Java/JavaScript etc. For that matter, can I have an instance of things other than
  structs? I think I can have instances of traits, and some other things I suspect. What are these things called? Should
  we always use the long form, "this is an instance of XYZ" or can we call them "objects"? 
  UPDATE: The question is squarely answered in chapter 17. *Object Oriented Rust Programming*: 
  > We’ve mentioned that in Rust, we refrain from calling structs and enums “objects” to distinguish them from other languages’ objects. In a struct or enum, the data in the struct fields and the behavior in impl blocks are separated, whereas in other languages, the data and behavior combined into one concept is often labeled an object. However, trait objects are more like objects in other languages in the sense that they combine data and behavior. But trait objects differ from traditional objects in that we can’t add data to a trait object.
* I don't grok the full range of syntax sugar when it comes to types that implement the `Copy` trait. I don't even know
  how to express this thought so I'll leave it at that. 
* I'm a fan of Rust's `Self` keyword. If Java had this, for example, we wouldn't have to type out the whole class name in the
  constructor definitions. This is especially annoying when the class name is very long and/or there are many constructor
  overloads.
* How does the automatic dereferencing syntax sugar (rather, compilation sugar magic?) work again? For example, in a function
  whose signature includes `&self`, you can author code like `self.xyz` (assuming there is a field `xyz`) and Rust knows
  what you mean and you don't need to do `*self.xyz` or whatever. UPDATE: I think this is called *deref coercion*
* I already forgot what slices are.
* I'm a fan of the codified error messages that the Rust compiler prints. For example, `error[E0106]: missing lifetime specifier`.
  It gives an identity to the error, which you can learn over time and become familiar with, and use as shared language when
  talking about Rust with other people. Nice.
* Rust is a hard-mode language. It is hard because of the low-level things like lifetimes and the ownership model. But to
  my surprise, it is also a high-level language because it offers high-level things like traits, lots and lots of syntax
  sugar, higher order functions, etc. I never realized a language could be both low- and high-level, but here it is. From
  now on, I will consider the range of lowness to highness that a language (or library) is instead of labelling a language
  as just high-level or low-level. C is only low-level. Python is only high-level (I think, I'm not a Python person). Java
  is high-level but if you consider Java contexts that instrument the JVM with special bytecode or use JNI then you might
  consider Java to span a bit lower level too. Rust ranges over low and high. It's interesting to think about. If you want
  short bit of code to remind yourself how low-level Rust is, just use this:
  ```
  &'a mut i32 // a mutable reference with an explicit lifetime
  ```
* Steve Klabnik said that most people think of Rust as not having a runtime, but it does have a runtime, it's just really
  small. Is Rust's "memory allocating and deallocating" work considered part of its runtime? Or is that all handled by
  the compiler? I'm out of my element here, but I'm curious. What is Rust's runtime? UPDATE: another component of the Rust
  runtime is the software machinery to handle dynamic dispatch. See [*Trait Objects Perform Dynamic Dispatch*](https://doc.rust-lang.org/stable/book/ch17-02-trait-objects.html#trait-objects-perform-dynamic-dispatch).
* I like the use of `::` for namespacing functions instead of using `.` like Java and other languages. In Java, we can fully
  qualify a method like `java.util.Date` but I like the use of `::` because it means that `.` isn't as overloaded in the
  language.
* The language crams in some expressive but cryptic (unless you've learned it) syntax:
  ```
  if let Some(s) = self.state.take() {
      self.state = Some(s.request_review())
  }
  ```
  No parentheses around the `if` boolean check. Pattern matching in the left-hand side. And what gets me is the `let` local
  variable declaration inside the `if` boolean check.
* I'm only now realizing that Rust doesn't have nulls?
* I should learn everything that the prelude imports (rather, *uses*).
* I don't know how to describe this, but Rust's generics don't allow dynamic values (well, except for boxed values with
  the `dyn` keyword) whereas Java's generics are more lenient. What is the "programming languages theory" name for this
  type of generics? The kind that *monomorphizes* at compile time? Whereas Java's doesn't do that, it has type erasure of
  the generic type parameters.
* DONE Re-format this notes page to use titles, sub-titles, sub-sub-titles (e.g. `##`, `###`) instead of indented bullets. The
  indented bullets are unwieldy with so much content.
* It's interesting that struct fields are private by default whereas in Java and Kotlin, fields are package-private and
  public by default, respectively. 
